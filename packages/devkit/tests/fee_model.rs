use stellar_devkit::simulation::fee_model::FeeModel;

#[test]
fn baseline_output_length() {
    let fees = FeeModel::baseline(10);
    assert_eq!(fees.len(), 10);
}

#[test]
fn baseline_values_in_range() {
    let fees = FeeModel::baseline(50);
    for fee in &fees {
        assert!(*fee >= 100.0 && *fee <= 1_000_000.0, "fee out of range: {fee}");
    }
}

#[test]
fn baseline_no_nans() {
    let fees = FeeModel::baseline(50);
    for fee in &fees {
        assert!(!fee.is_nan(), "unexpected NaN in baseline output");
    }
}
