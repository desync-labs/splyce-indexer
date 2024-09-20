import { Account, ShareAccount, TokenAccount } from "../../generated/schema";

export function updateAccountEntity(_authority: string, _tokenAccount: string, _shareAccount: string): void {

    let authorityAccount = Account.load(_authority);
    if (authorityAccount == null) {
        authorityAccount = new Account(_authority);
        authorityAccount.save();
    }

    let tokenAccount = TokenAccount.load(_tokenAccount);
    if (tokenAccount == null) {
        tokenAccount = new TokenAccount(_tokenAccount);
        tokenAccount.authority = _authority;
        tokenAccount.save();
    }

    let shareAccount = ShareAccount.load(_shareAccount);
    if (shareAccount == null) {
        shareAccount = new ShareAccount(_shareAccount);
        shareAccount.authority = _authority;
        shareAccount.save();
    }

    
}