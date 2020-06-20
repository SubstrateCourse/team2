1. Code filled in pallets/kitties/src/lib.rs.

2. I created a new HashMap "KittiesOwner" to save the mapping from kitty_id to Account_id.
The pseudo code:
function transfer_kitty(owner, kitty_id, dest) {
    Check owner is a valid signer;
    Check kitty_id exists in Kitties storage;
    Check owner owns kitty_id;
    OwnedKitties.remove((owner, kitty_id));
    OwnedKittiesCount(owner) -= 1;
    OwnedKitties.add((dest, kitty_id));
    OwnedKittiesCount(dest) += 1;
    KittiesOwner(kitty_id) = dest;
}

3. I created a new HashMap "Prices" to save the mapping from kitty_id to price. Also two methods "set_kitty_price" 
and "purchase_kitty" were added in decl_module!.
The pseudo code:
function set_kitty_price(owner, kitty_id, price) {
    Check owner is valid;
    Check kitty_id exists;
    Check owner owns kitty_id;
    Prices.insert((kitty_id, price));
}

function purchase_kitty(origin, kitty_id, offer_price) {
    Check origin is valid;
    Check kitty_id exists;
    Check origin != kitty owner;
    Check kitty price is already set;
    if offer_price >= kitty_price {
	transfer currency worth kitty_price from origin to owner;
        exchange the kitty_id from owner to origin
    } else {
        throw error offer_price is too low
    }
}
