use electoral_bond::polars_analysis::company_donation_percentage::company_donation_parcentage;
use electoral_bond::polars_analysis::donors_by_money::donors_by_money;
use electoral_bond::polars_analysis::party_by_money::party_encashment;
use electoral_bond::polars_analysis::party_money_percentage::party_money_parcentage;
use electoral_bond::xml_parse::parse_test::test_fn;
use electoral_bond::xml_parse::political_party::political_party_encashment;
fn main() {
    // let _data = test_fn();
    // let _party_encashment = political_party_encashment();

    // let _ = example1();
    // party_encashment();
    // donors_by_money();
    // let _ = party_money_parcentage();
    let _ = company_donation_parcentage().unwrap();
}
