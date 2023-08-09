import { BN, DataAndSlot } from '@drift-labs/sdk';
import { PublicKey } from '@solana/web3.js';
import { EventEmitter } from 'events';
import StrictEventEmitter from 'strict-event-emitter-types';

export const VAULT_PROGRAM_ID = new PublicKey(
	'VAULtLeTwwUxpwAw98E6XmgaDeQucKgV5UaiAuQ655D'
);

export class WithdrawUnit {
	static readonly SHARES = { shares: {} };
	static readonly TOKEN = { token: {} };
	static readonly SHARES_PERCENT = { sharesPercent: {} };
}

export type Vault = {
	name: number[];
	pubkey: PublicKey;
	manager: PublicKey;
	tokenAccount: PublicKey;
	userStats: PublicKey;
	user: PublicKey;
	delegate: PublicKey;
	liquidationDelegate: PublicKey;
	userShares: BN;
	totalShares: BN;
	lastFeeUpdateTs: BN;
	liquidationStartTs: BN;
	redeemPeriod: BN;
	totalWithdrawRequested: BN;
	maxTokens: BN;
	sharesBase: number;
	managementFee: BN;
	initTs: BN;
	netDeposits: BN;
	managerNetDeposits: BN;
	totalDeposits: BN;
	totalWithdraws: BN;
	managerTotalDeposits: BN;
	managerTotalWithdraws: BN;
	managerTotalFee: BN;
	managerTotalProfitShare: BN;
	minimumDeposit: BN;
	profitShare: number;
	hurdleRate: number;
	spotMarketIndex: number;
	bump: number;
	permissioned: boolean;
};

export type VaultDepositor = {
	vault: PublicKey;
	pubkey: PublicKey;
	authority: PublicKey;
	vaultShares: BN;
	lastWithdrawRequestShares: BN;
	lastWithdrawRequestValue: BN;
	lastWithdrawRequestTs: BN;
	lastValidTs: BN;
	netDeposits: BN;
	totalDeposits: BN;
	totalWithdraws: BN;
	cumulativeProfitShareAmount: BN;
	vaultSharesBase: number;
	profitShareFeePaid: BN;
	padding: number[];
};

export type VaultsProgramAccountBaseEvents = {
	update: void;
	error: (e: Error) => void;
};

export type VaultDepositorAccountEvents = {
	vaultDepositorUpdate: (payload: VaultDepositor) => void;
} & VaultsProgramAccountBaseEvents;

export type VaultAccountEvents = {
	vaultUpdate: (payload: Vault) => void;
} & VaultsProgramAccountBaseEvents;

export interface VaultsProgramAccountSubscriber<
	Account,
	AccountEvents extends VaultsProgramAccountBaseEvents
> {
	eventEmitter: StrictEventEmitter<EventEmitter, AccountEvents>;
	isSubscribed: boolean;

	subscribe(): Promise<boolean>;
	fetch(): Promise<void>;
	updateData(account: Account, slot: number): void;
	unsubscribe(): Promise<void>;
	getAccountAndSlot(): DataAndSlot<Account>;
}

export type VaultAccountSubscriber = VaultsProgramAccountSubscriber<
	Vault,
	VaultAccountEvents
>;

export type VaultDepositorAccountSubscriber = VaultsProgramAccountSubscriber<
	VaultDepositor,
	VaultDepositorAccountEvents
>;
