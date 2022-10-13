initSidebarItems({"struct":[["EthAddress","A generic representation of a local address. A resource id points to this. It may be a registry id (20 bytes) or a fungible asset type (in the future). Constrained to 32 bytes just as an upper bound to store efficiently."],["ItemId","A representation of ItemId for Uniques."],["RegistryId","A representation of registryID."],["TokenId",""],["TrancheWeight","A representation of a tranche weight, used to weight importance of a tranche"]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["Address","The address format for describing accounts."],["AllOfCouncil","All council members must vote yes to create this origin."],["AuraId","Aura consensus authority."],["Balance","Balance of an account."],["Block","A generic block for the node to use, as we can not commit to a specific Extrinsic format at this point. Runtimes will ensure Extrinsic are correctly decoded."],["BlockNumber","An index to a block."],["Bytes",""],["Bytes32",""],["CollectionId","A representation of CollectionId for Uniques"],["CouncilCollective","The council"],["EnsureRootOr",""],["FixedArray",""],["HalfOfCouncil","1/2 of all council members must vote yes to create this origin."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["IBalance","IBalance is the signed version of the Balance for orml tokens"],["Index","Index of a transaction in the chain."],["Moment","Moment type"],["OrderId","OrderId type we to identify order per epoch."],["PoolId","PoolId type we use."],["Salt",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["ThreeFourthOfCouncil","3/4 of all council members must vote yes to create this origin."],["TrancheId","A representation of a tranche identifier"],["TwoThirdOfCouncil","2/3 of all council members must vote yes to create this origin."]]});