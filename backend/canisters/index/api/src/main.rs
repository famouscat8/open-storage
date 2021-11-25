use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(index, allocated_bucket, query);
    generate_candid_method!(index, user, query);

    generate_candid_method!(index, add_user, update);
    generate_candid_method!(index, remove_accessor, update);
    generate_candid_method!(index, remove_user, update);
    generate_candid_method!(index, update_user, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}
