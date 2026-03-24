"use client";
import { createContext, useContext } from 'react';
import { ethers } from 'ethers';

export interface WalletState {
    provider: ethers.BrowserProvider | null;
    signer: ethers.JsonRpcSigner | null;
    address: string | null;
    connect: () => Promise<void>;
    disconnect: () => void;
}

export const WalletContext = createContext<WalletState | null>(null);

export const useWallet = () => {
    const context = useContext(WalletContext);
    if (!context) {
        throw new Error('useWallet must be used within a WalletProvider');
    }
    return context;
};
