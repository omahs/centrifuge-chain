initSidebarItems({"enum":[["CurrencyId",""],["Everything","A [`Contains`] implementation that contains every value."],["Nothing","A [`Contains`] implementation that contains no value."]],"macro":[["parameter_types","Create new implementations of the `Get` trait."]],"struct":[["AUSDPerSecond",""],["AccountIdToMultiLocation",""],["Ancestry",""],["BaseXcmWeight",""],["CanonicalNativePerSecond",""],["CheckingAccount",""],["CurrencyIdConvert","CurrencyIdConvert This type implements conversions from our `CurrencyId` type into `MultiLocation` and vice-versa. A currency locally is identified with a `CurrencyId` variant but in the network it is identified in the form of a `MultiLocation`, in this case a pair (Para-Id, Currency-Id)."],["KsmLocation",""],["KsmPerSecond",""],["MaxAssetsForTransfer",""],["MaxInstructions",""],["NativePerSecond",""],["NonZeroIssuance","Allow checking in assets that have issuance > 0."],["ParaId","Unique identifier of a parachain."],["ParachainMinFee",""],["RelayChainOrigin",""],["RelayNetwork",""],["SelfLocation","The `MultiLocation` identifying this very parachain"],["ToTreasury",""],["UnitWeightCost",""],["XcmConfig","The main XCM config This is where we configure the core of our XCM integrations: how tokens are transferred, how fees are calculated, what barriers we impose on incoming XCM messages, etc."]],"trait":[["Contains","A trait for querying whether a type can be said to “contain” a value."],["Get","A trait for querying a single value from a type."]],"type":[["Barrier","Barrier is a filter-like option controlling what messages are allows to be executed."],["FungiblesTransactor","Means for transacting the fungibles assets of ths parachain."],["LocalOriginToLocation","No local origins on this chain are allowed to dispatch XCM sends/executions."],["LocationToAccountId","Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used when determining ownership of accounts for asset transacting and when attempting to use XCM `Transact` in order to determine the dispatch Origin."],["Trader","Trader - The means of purchasing weight credit for XCM execution. We need to ensure we have at least one rule per token we want to handle or else the xcm executor won’t know how to charge fees for a transfer of said token."],["Weight","Numeric range of a transaction weight."],["XcmOriginToTransactDispatchOrigin","This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance, ready for dispatching a transaction with Xcm’s `Transact`. There is an `OriginKind` which can biases the kind of local `Origin` it will become."],["XcmRouter","The means for routing XCM messages which are not for local execution into the right message queues."]]});