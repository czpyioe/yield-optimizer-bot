use alloy::sol;

sol! {
    #[sol(rpc)]
    contract cTokenv3 {
        function getSupplyRate(uint utilization) override public view returns (uint64);
        function getUtilization() override public view returns (uint);
    }
}
