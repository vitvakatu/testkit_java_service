use exonum_testkit::{TestKit, TestKitBuilder};
use java_bindings::{
    exonum::{
        self,
        crypto::{self, PublicKey, SecretKey},
        messages::{BinaryForm, Message, RawTransaction, Signed},
        storage,
    },
    Config, InternalConfig, JavaServiceRuntime, JvmConfig, RuntimeConfig, ServiceConfig,
};

#[macro_use]
extern crate exonum_derive;
extern crate failure;
#[macro_use]
extern crate serde_derive;

const TEST_SERVICE_MODULE_NAME: &str = "com.exonum.binding.qaservice.ServiceModule";
const CLASSPATH: &str = include_str!("classpaths");
const COUNTERS_INDEX_NAME: &str = "ejb_qa_service__counters";
const DEFAULT_COUNTER_NAME: &str = "default";
const AFTER_COMMIT_COUNTER_NAME: &str = "after_commit_counter";
const TEST_COUNTER_NAME: &str = "test_counter";
const SERVICE_ID: u16 = 127;
const CREATE_COUNTER_TX_ID: u16 = 0;
const INCREMENT_COUNTER_TX_ID: u16 = 1;

mod proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateCounterTxBody")]
struct CreateCounterTx {
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::IncrementCounterTxBody")]
#[allow(non_snake_case)]
struct IncrementCounterTx {
    seed: u64,
    counterId: Vec<u8>,
}

#[test]
fn test_initial_conditions() {
    let testkit = create_testkit_with_qa_service();
    let default_counter = crypto::hash(DEFAULT_COUNTER_NAME.as_bytes());
    let after_commit_counter = crypto::hash(AFTER_COMMIT_COUNTER_NAME.as_bytes());
    assert_counter_value(&testkit, &default_counter, 0);
    assert_counter_value(&testkit, &after_commit_counter, 0);
}

#[test]
fn test_after_commit() {
    let mut testkit = create_testkit_with_qa_service();
    let after_commit_counter = crypto::hash(AFTER_COMMIT_COUNTER_NAME.as_bytes());
    assert_counter_value(&testkit, &after_commit_counter, 0);
    testkit.create_block();
    // TODO: WTF?
    assert_counter_value(&testkit, &after_commit_counter, 0);
    testkit.create_block();
    assert_counter_value(&testkit, &after_commit_counter, 0);
}

#[test]
fn test_creating_new_counter() {
    let mut testkit = create_testkit_with_qa_service();
    let keypair = crypto::gen_keypair();
    let test_counter = crypto::hash(TEST_COUNTER_NAME.as_bytes());

    let signed_tx = create_counter_tx(TEST_COUNTER_NAME, &keypair);
    testkit.create_block_with_transaction(signed_tx);
    assert_counter_value(&testkit, &test_counter, 0);

    let signed_tx = increment_counter_tx(&test_counter, &keypair, 0);
    testkit.create_block_with_transaction(signed_tx);
    assert_counter_value(&testkit, &test_counter, 1);
}

#[test]
fn test_incrementing_default() {
    let mut testkit = create_testkit_with_qa_service();
    let keypair = crypto::gen_keypair();
    let default_counter = crypto::hash(DEFAULT_COUNTER_NAME.as_bytes());
    assert_counter_value(&testkit, &default_counter, 0);

    let signed_tx = increment_counter_tx(&default_counter, &keypair, 0);
    testkit.create_block_with_transaction(signed_tx);
    assert_counter_value(&testkit, &default_counter, 1);
}

fn create_testkit_with_qa_service() -> TestKit {
    let service_config = ServiceConfig {
        module_name: TEST_SERVICE_MODULE_NAME.to_owned(),
        service_class_path: "".to_string(),
    };
    let runtime_config = RuntimeConfig {
        log_config_path: "".to_string(),
        port: 6000,
    };
    let jvm_config = JvmConfig {
        args_prepend: vec![],
        args_append: vec![],
        jvm_debug_socket: None,
    };
    let service_runtime = JavaServiceRuntime::get_or_create(
        Config {
            runtime_config,
            jvm_config,
            service_config,
        },
        InternalConfig {
            system_class_path: CLASSPATH.to_string(),
            system_lib_path: None,
        },
    );
    let service = service_runtime.create_service("", TEST_SERVICE_MODULE_NAME);
    TestKitBuilder::validator().with_service(service).create()
}

fn create_counter_tx(name: &str, (pk, sk): &(PublicKey, SecretKey)) -> Signed<RawTransaction> {
    let transaction = CreateCounterTx {
        name: name.to_string(),
    };
    let service_tx = exonum::messages::ServiceTransaction::from_raw_unchecked(
        CREATE_COUNTER_TX_ID,
        transaction.encode().unwrap(),
    );
    Message::sign_transaction(service_tx, SERVICE_ID, *pk, &sk)
}

fn increment_counter_tx(
    name: &crypto::Hash,
    (pk, sk): &(PublicKey, SecretKey),
    seed: u64,
) -> Signed<RawTransaction> {
    let transaction = IncrementCounterTx {
        seed,
        counterId: name.as_ref().to_vec(),
    };
    let service_tx = exonum::messages::ServiceTransaction::from_raw_unchecked(
        INCREMENT_COUNTER_TX_ID,
        transaction.encode().unwrap(),
    );
    Message::sign_transaction(service_tx, SERVICE_ID, *pk, &sk)
}

fn assert_counter_value(testkit: &TestKit, counter: &crypto::Hash, value: u64) {
    let view = testkit.snapshot();
    let index: storage::ProofMapIndex<_, crypto::Hash, u64> =
        storage::ProofMapIndex::new(COUNTERS_INDEX_NAME, view);
    assert_eq!(index.get(&counter), Some(value));
}
