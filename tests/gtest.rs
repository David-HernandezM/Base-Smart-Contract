use contract_client::{
    traits::*,
    contract_factory,
    contract_service::events::ContractServiceEvents
};
use fixture::{
    ADMIN_ID,
    CONTRACT_WASM_PATH,
    Fixture
};
use gstd::{errors::{
    ErrorReplyReason, 
    SimpleExecutionError
}, ActorId};
use sails_rs::{
    calls::*,
    errors::RtlError,
    events::*,
    futures::StreamExt,
    gtest::{
        Program, System,
        calls::{BlockRunMode, GTestRemoting},
    },
};

mod fixture;

#[tokio::test]
async fn hello_workd() {
    // Arrange

    let fixture = Fixture::new();
    let factory = fixture.contract_factory();
    let contract_id = factory
        .new()
        .send_recv(fixture.contract_code_id(), "123")
        .await
        .unwrap();

    let mut contract_client = fixture.contract_service_client();

    // Listen to contract service events
    let mut contract_listener = fixture.contract_service_listener();
    let mut contract_events = contract_listener
        .listen()
        .await
        .unwrap();

    // Act

    // Using generated client code for calling Contract service
    // using the `send_recv` method
    let result = contract_client
        .hello()
        .send_recv(contract_id)
        .await
        .unwrap();

    // Assert

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!(result, format!("Hello {:?}", ActorId::zero()));
    assert_eq!((contract_id, ContractServiceEvents::Hello(ActorId::zero())), event)

}

// use sails_rs::{calls::*, gtest::{calls::*, System}};

// use contract_client::traits::*;

// const ACTOR_ID: u64 = 42;

// #[tokio::test]
// async fn do_something_works() {
//     let system = System::new();
//     system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
//     system.mint_to(ACTOR_ID, 100_000_000_000_000);
//     let remoting = GTestRemoting::new(system, ACTOR_ID.into());

//     // Submit program code into the system
//     let program_code_id = remoting.system().submit_code(contract::WASM_BINARY);

//     let program_factory = contract_client::ContractFactory::new(remoting.clone());

//     let program_id = program_factory
//         .new() // Call program's constructor (see app/src/lib.rs:29)
//         .send_recv(program_code_id, b"salt")
//         .await
//         .unwrap();

//     let mut service_client = contract_client::Contract::new(remoting.clone());

//     let result = service_client
//         .do_something() // Call service's method (see app/src/lib.rs:14)
//         .send_recv(program_id)
//         .await
//         .unwrap();

//     assert_eq!(result, "Hello from Contract!".to_string());
// }

// #[tokio::test]
// async fn get_something_works() {
//     let system = System::new();
//     system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
//     system.mint_to(ACTOR_ID, 100_000_000_000_000);
//     let remoting = GTestRemoting::new(system, ACTOR_ID.into());

//     // Submit program code into the system
//     let program_code_id = remoting.system().submit_code(contract::WASM_BINARY);

//     let program_factory = contract_client::ContractFactory::new(remoting.clone());

//     let program_id = program_factory
//         .new() // Call program's constructor (see app/src/lib.rs:29)
//         .send_recv(program_code_id, b"salt")
//         .await
//         .unwrap();

//     let service_client = contract_client::Contract::new(remoting.clone());

//     let result = service_client
//         .get_something() // Call service's query (see app/src/lib.rs:19)
//         .recv(program_id)
//         .await
//         .unwrap();

//     assert_eq!(result, "Hello from Contract!".to_string());
// }
