use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("get_payload", |_| {
		//Ok(Value::String("hello".into()))
                Ok(Value::Array(vec![ "0xc409…1bb0".into() ,0.into(), 1.into(), 0.into(), 0.into(), 1.into()])) // 0,1,0,0,1
	});

	let server = ServerBuilder::new(io)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
		.start_http(&"0.0.0.0:9933".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}

