"""
IPython & Jupyter Interactive Notebook Extension for Oxide-Unsloth
Provides formatted widgets, rich markdown outputs, and live loss visualization.
"""

from typing import Any, Dict, Optional, List
import time


def display_training_header(model_name: str, lora_rank: int, epochs: int, device: str = "cuda"):
    """Renders formatted HTML/ANSI training banner in IPython/Jupyter."""
    try:
        from IPython.display import display, HTML
        html_content = f"""
        <div style="background-color: #1a1a2e; border: 1px solid #16213e; border-radius: 8px; padding: 12px; color: #e94560; font-family: monospace;">
            <h3 style="margin: 0; color: #00fff5;">🚀 Oxide-Unsloth Native Training Engine</h3>
            <p style="margin: 4px 0 0 0; color: #e0e0e0;"><b>Base Model:</b> {model_name} | <b>LoRA Rank:</b> {lora_rank} | <b>Epochs:</b> {epochs} | <b>Device:</b> {device}</p>
        </div>
        """
        display(HTML(html_content))
    except Exception:
        print(f"[Oxide-Unsloth IPython] Training: {model_name} | Rank: {lora_rank} | Epochs: {epochs}")


class IPythonTrainingCallback:
    """Live interactive progress tracker for Jupyter environments."""

    def __init__(self, total_steps: int = 100):
        self.total_steps = total_steps
        self.current_step = 0
        self.start_time = time.time()

    def on_step_end(self, step: int, loss: float, reward: Optional[float] = None):
        self.current_step = step
        elapsed = time.time() - self.start_time
        reward_str = f" | Reward: {reward:.3f}" if reward is not None else ""
        try:
            from IPython.display import clear_output
            clear_output(wait=True)
            print(f"[{step}/{self.total_steps}] Loss: {loss:.4f}{reward_str} | Elapsed: {elapsed:.1f}s")
        except Exception:
            print(f"[{step}/{self.total_steps}] Loss: {loss:.4f}{reward_str}")


def load_ipython_extension(ipython: Any):
    """Magic extension hook for `%load_ext oxide_unsloth`."""
    print("[Oxide-Unsloth] IPython extension loaded successfully.")
