module 0x1::SetheumNFT {
    use std::signer;
    use std::vector;

    struct NFT has store, copy, drop {
        id: u64,
        metadata: vector<u8>,
    }

    struct Collection has key {
        items: vector<NFT>,
        next_id: u64,
    }

    public fun create_collection(account: &signer) {
        move_to(account, Collection {
            items: vector::empty<NFT>(),
            next_id: 0,
        });
    }

    public fun mint(account: &signer, recipient: address, metadata: vector<u8>) acquires Collection {
        let collection = borrow_global_mut<Collection>(signer::address_of(account));
        let nft = NFT {
            id: collection.next_id,
            metadata,
        };
        vector::push_back(&mut collection.items, nft);
        collection.next_id = collection.next_id + 1;
        
        // In real Move, you'd move the NFT to the recipient's account
    }

    public fun get_nft_metadata(owner: address, id: u64): vector<u8> acquires Collection {
        let collection = borrow_global<Collection>(owner);
        let i = 0;
        let len = vector::length(&collection.items);
        while (i < len) {
            let nft = vector::borrow(&collection.items, i);
            if (nft.id == id) {
                return *&nft.metadata
            };
            i = i + 1;
        };
        vector::empty<u8>()
    }
}
