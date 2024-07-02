use proto::calculator_client::CalculatorClient;
use std::error::Error;
use tonic::Request;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    
    let mut calc_client = CalculatorClient::connect(url).await?;
    
    let add_request = proto::CalculationRequest { a: 4, b: 3 };
    let request = Request::new(add_request);
    let add_response = calc_client.add(request).await?;
    
    println!("Add Response: {:?}", add_response.get_ref().result);
    
    let divide_request = proto::CalculationRequest { a: 10, b: 5 };
    let request = Request::new(divide_request);
    let divide_response = calc_client.divide(request).await?;
    
    println!("Divide Response: {:?}", divide_response.get_ref().result);
     
    Ok(())
}
