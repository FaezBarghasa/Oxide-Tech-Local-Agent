from algorithm import vectorize
from sys.info import simdwidthof

# Fast SIMD-accelerated cosine distance between two float32 vectors
fn cosine_similarity_simd(a: DTypePointer[DType.float32], b: DTypePointer[DType.float32], size: Int) -> Float32:
    alias simd_width = simdwidthof[DType.float32]()
    var dot_product: Float32 = 0.0
    var norm_a: Float32 = 0.0
    var norm_b: Float32 = 0.0

    @parameter
    fn compute_vectorized[simd_w: Int](idx: Int):
        let va = a.load[width=simd_w](idx)
        let vb = b.load[width=simd_w](idx)
        dot_product += (va * vb).reduce_add()
        norm_a += (va * va).reduce_add()
        norm_b += (vb * vb).reduce_add()

    vectorize[compute_vectorized, simd_width](size)

    if norm_a == 0.0 or norm_b == 0.0:
        return 0.0

    # Return cosine similarity
    return dot_product / ((norm_a ** 0.5) * (norm_b ** 0.5))

fn main():
    print("Mojo SIMD vector distance module loaded successfully.")
