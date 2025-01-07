use vertin::pbr::SchlickFresnelBaking;

fn main() {
    let baker = SchlickFresnelBaking::new(64, 8192);

    let roughness = 63.5 / 64.0;

    // let result = baker.bake_one_sample(63.5 / 64.0, roughness * roughness);
    let result = baker.bake_one_sample(1.0, 1.0);
    // let result = baker.bake_one_sample_use_normal_distribution(1.0, 1.0);
    println!("{:?}", result);
}