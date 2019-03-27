use mac::*;

// Define the access levels.

enum Official {}
impl AccessLevel for Official {}
enum Secret {}
impl AccessLevel for Secret {}
impl Above<Official> for Secret {}
enum TopSecret {}
impl AccessLevel for TopSecret {}
impl Above<Official> for TopSecret {}
impl Above<Secret> for TopSecret {}

fn main() {
    // Passing to same access level is fine
    send_data::<Official, Official>();
    send_data::<Secret, Secret>();
    send_data::<Secret, Secret>();

    // Passing to higher access level is fine
    send_data::<Official, Secret>();
    send_data::<Secret, TopSecret>();
    send_data::<Official, TopSecret>();

    // Passing to lower access level is incorrect
    send_data::<Secret, Official>();
    send_data::<TopSecret, Secret>();
    send_data::<TopSecret, Official>();
}

fn send_data<A: AccessLevel, B: AccessLevel + Above<A>>() {
    let data: Mac<A, _> = Mac::pure(42);

    let operation: Mac<B, _> = Mac::pure(|n| n + 1);

    let result: Mac<B, _> = data.map(operation);

    println!("Got value: {}", result.value());
}
