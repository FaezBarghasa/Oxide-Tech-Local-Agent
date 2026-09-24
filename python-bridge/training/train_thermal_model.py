#!/usr/bin/env python3
"""
Oxide-Tech Circuit Forge: Graph Neural Network (GNN) Thermal Solver
Predicts steady-state junction temperatures and PCB heat distribution from
component power dissipation arrays and layout topology.
"""

import json
import os
import sys
import numpy as np

try:
    import torch
    import torch.nn as nn
    import torch.nn.functional as F
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False


class ComponentThermalGNN(nn.Module if TORCH_AVAILABLE else object):
    """
    Message-passing neural network for conjugate heat transfer on PCB topologies.
    Node features: [power_dissipation_watts, pos_x_mm, pos_y_mm, package_area_mm2, r_theta_ja]
    Edge features: [copper_distance_mm, copper_width_mm, thermal_conductivity_w_mk]
    Target output: [junction_temperature_c, surface_temperature_c]
    """
    def __init__(self, node_in_dim=5, edge_in_dim=3, hidden_dim=64):
        if not TORCH_AVAILABLE:
            return
        super().__init__()
        self.node_encoder = nn.Linear(node_in_dim, hidden_dim)
        self.edge_encoder = nn.Linear(edge_in_dim, hidden_dim)

        # Message passing layers
        self.msg_fc1 = nn.Linear(hidden_dim * 2 + hidden_dim, hidden_dim)
        self.node_update1 = nn.GRUCell(hidden_dim, hidden_dim)

        self.msg_fc2 = nn.Linear(hidden_dim * 2 + hidden_dim, hidden_dim)
        self.node_update2 = nn.GRUCell(hidden_dim, hidden_dim)

        # Temperature readout head
        self.temp_head = nn.Sequential(
            nn.Linear(hidden_dim, 32),
            nn.ReLU(),
            nn.Linear(32, 2)
        )

    def forward(self, x, edge_index, edge_attr):
        if not TORCH_AVAILABLE:
            raise RuntimeError("PyTorch is required for GNN thermal forward pass.")

        h = F.relu(self.node_encoder(x))
        e = F.relu(self.edge_encoder(edge_attr))

        # Iteration 1
        src, dst = edge_index[0], edge_index[1]
        msg1 = F.relu(self.msg_fc1(torch.cat([h[src], h[dst], e], dim=-1)))
        agg1 = torch.zeros_like(h)
        agg1.index_add_(0, dst, msg1)
        h = self.node_update1(agg1, h)

        # Iteration 2
        msg2 = F.relu(self.msg_fc2(torch.cat([h[src], h[dst], e], dim=-1)))
        agg2 = torch.zeros_like(h)
        agg2.index_add_(0, dst, msg2)
        h = self.node_update2(agg2, h)

        # Predict [T_junction, T_surface]
        temperatures = self.temp_head(h)
        return temperatures


def simulate_thermal_distribution(components_data, ambient_temp_c=25.0):
    """
    Solve thermal dissipation for a list of components:
    components_data: list of dicts with keys:
      - ref_des: str
      - power_watts: float
      - x_mm: float
      - y_mm: float
      - r_theta_ja: float (C/W)
    Returns: dict mapping ref_des -> estimated junction temp (C)
    """
    results = {}
    total_power = sum(c.get("power_watts", 0.0) for c in components_data)

    for comp in components_data:
        ref_des = comp.get("ref_des", "U?")
        power = float(comp.get("power_watts", 0.0))
        r_ja = float(comp.get("r_theta_ja", 45.0))

        # Self-heating + mutual heating approximation from board average
        self_heating = power * r_ja
        mutual_heating = (total_power - power) * 3.5  # Coupled substrate dissipation
        t_junction = ambient_temp_c + self_heating + mutual_heating

        results[ref_des] = {
            "power_watts": power,
            "junction_temp_c": round(t_junction, 2),
            "max_safe_temp_c": 125.0,
            "passes_derating": t_junction < 105.0
        }

    return results


def main():
    if len(sys.argv) > 1 and os.path.exists(sys.argv[1]):
        with open(sys.argv[1], "r") as f:
            data = json.load(f)
    else:
        # Default test netlist component power profile
        data = [
            {"ref_des": "U1", "power_watts": 1.8, "x_mm": 25.0, "y_mm": 25.0, "r_theta_ja": 32.0},
            {"ref_des": "U2", "power_watts": 0.4, "x_mm": 15.0, "y_mm": 10.0, "r_theta_ja": 65.0},
            {"ref_des": "Q1", "power_watts": 0.9, "x_mm": 35.0, "y_mm": 20.0, "r_theta_ja": 40.0},
            {"ref_des": "R1", "power_watts": 0.1, "x_mm": 20.0, "y_mm": 30.0, "r_theta_ja": 120.0},
        ]

    predictions = simulate_thermal_distribution(data)
    print(json.dumps(predictions, indent=2))


if __name__ == "__main__":
    main()
