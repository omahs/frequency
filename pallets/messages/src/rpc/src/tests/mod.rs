mod rpc_mock;

use super::rpc::*;
use rpc_mock::*;

use common_primitives::node::BlockNumber;
use pallet_messages_runtime_api::MessagesRuntimeApi;
use std::sync::Arc;
use substrate_test_runtime_client::runtime::Block;

const SCHEMA_ID_EMPTY: u16 = 1;
const SCHEMA_ID_HAS_MESSAGES: u16 = 2;

fn test_messages() -> Vec<MessageResponse> {
	vec![
		MessageResponse {
			payload: None,
			msa_id: None,
			provider_msa_id: 1,
			index: 0,
			block_number: 1,
			cid: Some(vec![0, 1, 2, 3]),
			payload_length: Some(42),
		},
		MessageResponse {
			payload: None,
			msa_id: None,
			provider_msa_id: 1,
			index: 1,
			block_number: 1,
			cid: Some(vec![0, 1, 2, 3]),
			payload_length: Some(42),
		},
	]
}

sp_api::mock_impl_runtime_apis! {
	impl MessagesRuntimeApi<Block> for TestRuntimeApi {
		fn get_schema_by_id(schema_id: SchemaId) -> Option<SchemaResponse> {
			match schema_id {
				SCHEMA_ID_EMPTY | SCHEMA_ID_HAS_MESSAGES => Some(SchemaResponse {
					schema_id,
					model: b"schema".to_vec(),
					model_type: ModelType::AvroBinary,
					payload_location: PayloadLocation::default(),
				}),
				_ => None,
			}
		}

		fn get_messages_by_schema_and_block(schema_id: SchemaId, _schema_payload_location: PayloadLocation, _block_number: BlockNumber) ->
			Vec<MessageResponse> {
				match schema_id {
					SCHEMA_ID_HAS_MESSAGES => test_messages(),
					_ => vec![]
				}
			}
	}
}

type GetMessagesBySchemaResult = Result<
	common_primitives::messages::BlockPaginationResponse<
		common_primitives::messages::MessageResponse,
	>,
	jsonrpsee::core::Error,
>;

#[tokio::test]
async fn get_messages_by_schema_with_invalid_request_should_panic() {
	let client = Arc::new(TestApi {});
	let api = MessagesHandler::new(client);

	let result: GetMessagesBySchemaResult = api.get_messages_by_schema_id(
		SCHEMA_ID_EMPTY, // Schema Id
		BlockPaginationRequest { from_block: 1, to_block: 2, from_index: 0, page_size: 0 },
	);

	assert_eq!(true, result.is_err());
	assert_eq!("Custom error: InvalidPaginationRequest", result.unwrap_err().to_string());
}

#[tokio::test]
async fn get_messages_by_schema_with_bad_schema_id_should_err() {
	let client = Arc::new(TestApi {});
	let api = MessagesHandler::new(client);

	let result: GetMessagesBySchemaResult = api.get_messages_by_schema_id(
		0, // Schema Id
		BlockPaginationRequest { from_block: 1, to_block: 5, from_index: 0, page_size: 10 },
	);

	assert_eq!(true, result.is_err());
	// assert_eq!("RPC call failed: ErrorObject { code: ServerError(300), message: \"Api Error\", data: Some(RawValue(\"InvalidSchemaId\")) }", result.unwrap_err().to_string());
	assert_eq!("Custom error: InvalidSchemaId", result.unwrap_err().to_string());
}

#[tokio::test]
async fn get_messages_by_schema_with_success() {
	let client = Arc::new(TestApi {});
	let api = MessagesHandler::new(client);

	let result: GetMessagesBySchemaResult = api.get_messages_by_schema_id(
		SCHEMA_ID_HAS_MESSAGES, // Schema Id
		BlockPaginationRequest { from_block: 1, to_block: 5, from_index: 0, page_size: 2 },
	);

	assert_eq!(true, result.is_ok());
	let response = result.unwrap();
	// Because the page size is set to 2, we only get one set of messages
	assert_eq!(test_messages(), response.content);
	// There is more because we haven't done all the blocks yet
	assert_eq!(true, response.has_next);
	// Next block is 2 and index 0 as block 1 just has those two messages
	assert_eq!(Some(2), response.next_block);
	assert_eq!(Some(0), response.next_index);
}
