
#[derive(Debug)]
pub struct ContractOne {
    input_data: String,
    output_data: Option<Result<String, String>>
}
#[derive(Debug)]
pub struct ContractTwo {
    input_data: String,
    output_data: Option<Result<String, String>>
}

#[derive(Debug)]
pub enum ContractHandler {
    ContractOne(ContractOne),
    ContractTwo(ContractTwo),
}


fn handle_contract_one(mut contract: ContractOne) -> ContractOne {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}
fn handle_contract_two(mut contract: ContractTwo) -> ContractTwo {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}

#[macro_export]
macro_rules! register_contract_routes {
    (
        $handler_enum:ident, 
        $fn_name:ident, 
        $( $contract:ident => $handler_fn:path ),*) => {
            pub fn $fn_name(received_msg: $handler_enum) -> $handler_enum {
                match received_msg {
                    $(
                        $handler_enum::$contract(inner) => {
                            let executed_contract = $handler_fn(inner);
                            return $handler_enum::$contract(
                                executed_contract
                            )
                        }
                    )*
                }
            }            
    };
}


register_contract_routes!(
    ContractHandler, 
    handle_contract, 
    ContractOne => handle_contract_one, 
    ContractTwo => handle_contract_two
);


fn main() {
    let contract_one = ContractOne {
        input_data: "Contract One".to_string(),
        output_data: None
    };
    let outcome = handle_contract(
        ContractHandler::ContractOne(contract_one)
    );
    println!("{:?}", outcome);
}

