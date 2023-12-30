use scale::Encode as _;
#[allow(dead_code)]
pub const CODE_HASH: [u8; 32] = [
    207u8, 62u8, 188u8, 75u8, 164u8, 98u8, 81u8, 186u8, 165u8, 245u8, 181u8, 227u8, 59u8, 26u8,
    28u8, 84u8, 209u8, 51u8, 14u8, 177u8, 141u8, 195u8, 164u8, 71u8, 127u8, 34u8, 166u8, 45u8,
    239u8, 194u8, 237u8, 191u8,
];
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct SubscriptionPlan {
    pub relayer_id: ink_primitives::AccountId,
    pub price_per_month: u128,
    pub price_per_week: u128,
    pub price_per_year: u128,
    pub subscribers: Vec<ink_primitives::AccountId>,
    pub plan_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct Subscription {
    pub subscriber: ink_primitives::AccountId,
    pub relayer_id: ink_primitives::AccountId,
    pub duration: SubscriptionDuration,
    pub start_date: u64,
    pub expiry_date: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub enum SubscriptionDuration {
    Month(),
    Week(),
    Year(),
    Unknown(),
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct Report {
    pub reporter: ink_primitives::AccountId,
    pub relayer: ink_primitives::AccountId,
    pub challenged: bool,
    pub report_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct NostrContract {
    pub owner: ink_primitives::AccountId,
    pub subscription_plans: Vec<SubscriptionPlan>,
    pub subscriptions: Vec<Subscription>,
    pub reports: Vec<Report>,
    pub next_report_id: u64,
    pub challenger: Option<ink_primitives::AccountId>,
    pub nostr_public_keys: Vec<(ink_primitives::AccountId, Vec<u8>)>,
    pub next_plan_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct SubscriptionPlanInfo {
    pub relayer_id: ink_primitives::AccountId,
    pub price_per_month: u128,
    pub price_per_week: u128,
    pub price_per_year: u128,
    pub subscribers: Vec<SubscriberInfoEntry>,
    pub plan_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct SubscriberInfoEntry {
    pub sub_id: ink_primitives::AccountId,
    pub nostr_pubkey: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub struct SubscriberInfo {
    pub ok: Vec<SubscriberInfoEntry>,
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub enum NoChainExtension {}
pub mod event {
    #[allow(dead_code, clippy::large_enum_variant)]
    #[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
    pub enum Event {
        Subscribed {
            subscriber: ink_primitives::AccountId,
            relayer: ink_primitives::AccountId,
            amount: u128,
        },
        ReportEvent {
            relayer: ink_primitives::AccountId,
            signature: Vec<u8>,
            report_id: u64,
        },
        Challenged {
            reporter: ink_primitives::AccountId,
            report_id: u64,
        },
        SubscriptionPlanNotFound {
            not_found_id: ink_primitives::AccountId,
            relayer_id: ink_primitives::AccountId,
            subscriber: ink_primitives::AccountId,
        },
        StartDateTimeSet {
            start_date: u64,
        },
        ExpiryDateTimeSet {
            expiry_date: u64,
        },
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    account_id: ink_primitives::AccountId,
}
impl From<ink_primitives::AccountId> for Instance {
    fn from(account_id: ink_primitives::AccountId) -> Self {
        Self { account_id }
    }
}
impl From<Instance> for ink_primitives::AccountId {
    fn from(instance: Instance) -> Self {
        instance.account_id
    }
}
impl ink_wrapper_types::EventSource for Instance {
    type Event = event::Event;
}
impl Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn new() -> ink_wrapper_types::InstantiateCall<Self> {
        let data = vec![155u8, 174u8, 157u8, 94u8];
        ink_wrapper_types::InstantiateCall::new(CODE_HASH, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn create_subscription_plan(
        &self,
        price_per_week: u128,
        price_per_month: u128,
        price_per_year: u128,
    ) -> ink_wrapper_types::ExecCall {
        let data = {
            let mut data = vec![240u8, 167u8, 141u8, 179u8];
            price_per_week.encode_to(&mut data);
            price_per_month.encode_to(&mut data);
            price_per_year.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn subscribe_to_plan(
        &self,
        relayer_id: ink_primitives::AccountId,
        nostr_public_key: Vec<u8>,
        duration: SubscriptionDuration,
    ) -> ink_wrapper_types::ExecCall {
        let data = {
            let mut data = vec![28u8, 20u8, 211u8, 229u8];
            relayer_id.encode_to(&mut data);
            nostr_public_key.encode_to(&mut data);
            duration.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_subscription_plans(
        &self,
    ) -> ink_wrapper_types::ReadCall<
        Result<Vec<SubscriptionPlanInfo>, ink_wrapper_types::InkLangError>,
    > {
        let data = vec![146u8, 19u8, 153u8, 72u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn report(
        &self,
        relayer: ink_primitives::AccountId,
        signature: Vec<u8>,
    ) -> ink_wrapper_types::ExecCall {
        let data = {
            let mut data = vec![243u8, 15u8, 70u8, 76u8];
            relayer.encode_to(&mut data);
            signature.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn challenge_report(&self, report_id: u64) -> ink_wrapper_types::ExecCall {
        let data = {
            let mut data = vec![147u8, 168u8, 80u8, 149u8];
            report_id.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_subscription(
        &self,
        subscriber: ink_primitives::AccountId,
    ) -> ink_wrapper_types::ReadCall<Result<Option<Subscription>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![69u8, 26u8, 172u8, 112u8];
            subscriber.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_subscribers(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<SubscriberInfo, ink_wrapper_types::InkLangError>> {
        let data = vec![22u8, 183u8, 254u8, 116u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_report(
        &self,
        report_id: u64,
    ) -> ink_wrapper_types::ReadCall<Result<Option<Report>, ink_wrapper_types::InkLangError>> {
        let data = {
            let mut data = vec![235u8, 159u8, 96u8, 150u8];
            report_id.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
}
