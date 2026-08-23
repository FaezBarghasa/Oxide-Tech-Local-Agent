from algorithm import vectorize
from memory import UnsafePointer
from sys.info import simdwidthof

alias type = Float32
alias simd_width = simdwidthof[type]()

struct HybridScoreEngine:
    var count: Int
    var alpha: Float32
    var beta: Float32
    var gamma: Float32

    fn __init__(inout self, count: Int, alpha: Float32, beta: Float32, gamma: Float32):
        self.count = count
        self.alpha = alpha
        self.beta = beta
        self.gamma = gamma

    fn fuse_scores_simd(
        self,
        cosine_sims: UnsafePointer[Float32],
        pagerank_scores: UnsafePointer[Float32],
        depth_penalties: UnsafePointer[Float32],
        output_scores: UnsafePointer[Float32],
    ):
        @parameter
        fn vectorized_calc[simd_w: Int](idx: Int):
            let c_sim = cosine_sims.load[width=simd_w](idx)
            let p_rank = pagerank_scores.load[width=simd_w](idx)
            let d_pen = depth_penalties.load[width=simd_w](idx)
            
            let final_val = (c_sim * self.alpha) + (p_rank * self.beta) - (d_pen * self.gamma)
            output_scores.store[width=simd_w](idx, final_val)

        vectorize[vectorized_calc, simd_width](self.count)

# Exported C-ABI for direct Rust FFI integration
@export
fn mojo_fuse_graph_scores(
    count: Int,
    alpha: Float32,
    beta: Float32,
    gamma: Float32,
    cosine_ptr: UnsafePointer[Float32],
    pagerank_ptr: UnsafePointer[Float32],
    depth_ptr: UnsafePointer[Float32],
    out_ptr: UnsafePointer[Float32],
):
    let engine = HybridScoreEngine(count, alpha, beta, gamma)
    engine.fuse_scores_simd(cosine_ptr, pagerank_ptr, depth_ptr, out_ptr)
