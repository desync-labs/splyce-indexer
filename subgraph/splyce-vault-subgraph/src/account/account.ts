import { Account, TokenWallet } from "../../generated/schema";

export function updateAccountEntity(_authority: string, _tokenAccount: string, _shareAccount: string): Account {

    let authorityAccount = Account.load(_authority);
    if (authorityAccount == null) {
        authorityAccount = new Account(_authority);
        authorityAccount.save();
    }

    let tokenAccount = TokenWallet.load(_tokenAccount);
    if (tokenAccount == null) {
        tokenAccount = new TokenWallet(_tokenAccount);
        tokenAccount.authority = _authority;
        tokenAccount.save();
    }

    let shareAccount = TokenWallet.load(_shareAccount);
    if (shareAccount == null) {
        shareAccount = new TokenWallet(_shareAccount);
        shareAccount.authority = _authority;
        shareAccount.save();
    }

    return authorityAccount as Account;
    
}