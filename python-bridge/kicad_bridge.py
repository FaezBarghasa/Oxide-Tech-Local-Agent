import subprocess
import os
import bridge_pb2
import bridge_pb2_grpc

class KiCadBridgeServicer(bridge_pb2_grpc.KiCadServiceServicer):
    def CreateSchematic(self, request, context):
        try:
            os.makedirs(request.output_dir, exist_ok=True)
            out_path = os.path.join(request.output_dir, f"{request.project_name}.kicad_sch")
            
            # Generate programmatic KiCad schematic format
            with open(out_path, "w", encoding="utf-8") as f:
                f.write(f'(kicad_sch (version 20231120) (generator "oxide-agent")\n')
                f.write(f'  (paper "A4")\n')
                f.write(f'  (title_block (title "{request.project_name}"))\n')
                for i, comp in enumerate(request.components):
                    f.write(f'  (symbol (lib_id "{comp.symbol}") (at {50 + i * 20} {50 + i * 20}) (unit 1)\n')
                    f.write(f'    (property "Reference" "{comp.ref}")\n')
                    f.write(f'    (property "Value" "{comp.value}")\n')
                    f.write(f'  )\n')
                for net in request.nets:
                    f.write(f'  (wire (pts (xy 0 0) (xy 10 10)) (stroke (width 0) (type default))\n')
                    f.write(f'    (property "NetName" "{net.net_name}")\n')
                    f.write(f'  )\n')
                f.write(f')\n')
                
            return bridge_pb2.SchematicResponse(
                success=True,
                output_path=out_path,
                error_message=""
            )
        except Exception as e:
            return bridge_pb2.SchematicResponse(
                success=False,
                output_path="",
                error_message=str(e)
            )

    def RunDRC(self, request, context):
        if not os.path.exists(request.pcb_path):
            return bridge_pb2.DRCResponse(
                passed=False,
                violations=[f"File not found: {request.pcb_path}"]
            )
        cmd = f"kicad-cli pcb drc --format json -o drc_out.json {request.pcb_path}"
        res = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        passed = (res.returncode == 0)
        violations = [res.stderr] if res.stderr else []
        return bridge_pb2.DRCResponse(passed=passed, violations=violations)
