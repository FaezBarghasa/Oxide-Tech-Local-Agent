# Mojo intent routing score evaluator using SIMD weights
from algorithm import vectorize

fn route_intent_score(features: DTypePointer[DType.float32], weights: DTypePointer[DType.float32], count: Int) -> Float32:
    var score: Float32 = 0.0

    @parameter
    fn accumulate[w: Int](i: Int):
        let f = features.load[width=w](i)
        let wt = weights.load[width=w](i)
        score += (f * wt).reduce_add()

    vectorize[accumulate, 8](count)
    return score

fn main():
    print("Mojo intent router ready.")
