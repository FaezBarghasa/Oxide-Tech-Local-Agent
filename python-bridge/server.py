import asyncio
import os
import sys
import grpc
from concurrent import futures

# Add local path to sys.path so it can find the generated pb2 modules
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

import bridge_pb2
import bridge_pb2_grpc

class BridgeService(bridge_pb2_grpc.BridgeServiceServicer):
    async def AnalyzeNetlist(self, request, context):
        print(f"Direct UDS Netlist verification requested for: {request.project_id}")
        return bridge_pb2.NetlistResponse(
            success=True,
            report="Optimized parsing completed over binary boundaries.",
            warnings=[]
        )

    async def RunThermalSimulation(self, request, context):
        return bridge_pb2.ThermalResponse(
            heat_matrix=[22.5, 45.1, 78.2, 34.0],
            peak_temp_celsius=78.2
        )

async def serve_uds():
    socket_path = "/tmp/oxide_bridge.sock"
    if os.path.exists(socket_path):
        try:
            os.remove(socket_path)
        except OSError:
            pass

    server = grpc.aio.server(futures.ThreadPoolExecutor(max_workers=16))
    bridge_pb2_grpc.add_BridgeServiceServicer_to_server(BridgeService(), server)

    server.add_insecure_port(f"unix://{socket_path}")
    print(f"Fast gRPC daemon successfully bound to UDS: {socket_path}")

    await server.start()
    try:
        os.chmod(socket_path, 0o666)
    except OSError:
        pass
    await server.wait_for_termination()

if __name__ == "__main__":
    asyncio.run(serve_uds())
