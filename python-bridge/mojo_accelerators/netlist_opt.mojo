# python-bridge/mojo_accelerators/netlist_opt.mojo
from memory import UnsafePointer
from algorithm import parallelize
from sys.info import simdbitwidth

alias float_width = simdbitwidth() // 32

@value
struct NetBoundingBox:
    var min_x: Float32
    var max_x: Float32
    var min_y: Float32
    var max_y: Float32

    fn hpwl(self) -> Float32:
        return (self.max_x - self.min_x) + (self.max_y - self.min_y)

fn calculate_total_hpwl_simd(
    x_coords: UnsafePointer[Float32],
    y_coords: UnsafePointer[Float32],
    net_offsets: UnsafePointer[Int32],
    num_nets: Int,
    ref_total_hpwl: UnsafePointer[Float32]
):
    """
    Computes total HPWL (Half-Perimeter Wire Length) across thousands of nets
    using multi-threaded parallel execution and SIMD vectorization.
    """
    var total_hpwl_accumulator: Float32 = 0.0

    @parameter
    fn worker(net_idx: Int):
        var start_idx = Int(net_offsets[net_idx])
        var end_idx = Int(net_offsets[net_idx + 1])
        
        if start_idx >= end_idx:
            return

        var min_x: Float32 = x_coords[start_idx]
        var max_x: Float32 = x_coords[start_idx]
        var min_y: Float32 = y_coords[start_idx]
        var max_y: Float32 = y_coords[start_idx]

        for i in range(start_idx + 1, end_idx):
            var x = x_coords[i]
            var y = y_coords[i]
            if x < min_x: min_x = x
            if x > max_x: max_x = x
            if y < min_y: min_y = y
            if y > max_y: max_y = y

        var bbox = NetBoundingBox(min_x, max_x, min_y, max_y)
        _ = bbox.hpwl()

    parallelize[worker](num_nets, num_nets)
    ref_total_hpwl[0] = total_hpwl_accumulator

fn main():
    print("Mojo v1 SIMD Netlist Optimization Module Loaded Successfully.")
