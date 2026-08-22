import subprocess
import os
import bridge_pb2
import bridge_pb2_grpc

class CADServiceServicer(bridge_pb2_grpc.CADServiceServicer):
    def ConvertStepToGltf(self, request, context):
        if not os.path.exists(request.step_path):
            return bridge_pb2.StepConvertResponse(
                success=False,
                error_message=f"STEP file does not exist: {request.step_path}"
            )
            
        cmd = f"freecadcmd -c \"import Part, Mesh; s=Part.Shape(); s.read('{request.step_path}'); m=Mesh.Mesh(s.tessellate(0.1)); m.write('{request.gltf_path}')\""
        res = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        if res.returncode == 0:
            return bridge_pb2.StepConvertResponse(success=True, error_message="")
        return bridge_pb2.StepConvertResponse(
            success=False,
            error_message=res.stderr if res.stderr else "FreeCAD conversion failed"
        )
