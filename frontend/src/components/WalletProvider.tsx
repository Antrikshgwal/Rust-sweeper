"use client";
import { useState, useEffect, useCallback } from 'react';
import { ethers } from 'ethers';
import { WalletContext, WalletState } from '@/contexts/WalletContext';

interface WalletProviderProps {
    children: React.ReactNode;
}

export const WalletProvider = ({ children }: WalletProviderProps) => {
    const [provider, setProvider] = useState<ethers.BrowserProvider | null>(null);
    const [signer, setSigner] = useState<ethers.JsonRpcSigner | null>(null);
    const [address, setAddress] = useState<string | null>(null);

    const connect = useCallback(async () => {
        if (typeof window.ethereum !== 'undefined') {
            try {
                const browserProvider = new ethers.BrowserProvider(window.ethereum);
                await browserProvider.send('eth_requestAccounts', []);
                const signer = await browserProvider.getSigner();
                const address = await signer.getAddress();
                setProvider(browserProvider);
                setSigner(signer);
                setAddress(address);
            } catch (error) {
                console.error("Failed to connect wallet:", error);
            }
        } else {
            console.error("MetaMask is not installed");
        }
    }, []);

    const disconnect = useCallback(() => {
        setProvider(null);
        setSigner(null);
        setAddress(null);
    }, []);

    useEffect(() => {
        if (window.ethereum) {
            const handleAccountsChanged = (accounts: string[]) => {
                if (accounts.length === 0) {
                    disconnect();
                } else {
                    connect();
                }
            };

            window.ethereum.on('accountsChanged', handleAccountsChanged);

            return () => {
                if (window.ethereum.removeListener) {
                    window.ethereum.removeListener('accountsChanged', handleAccountsChanged);
                }
            };
        }
    }, [connect, disconnect]);

    const value: WalletState = { provider, signer, address, connect, disconnect };

    return (
        <WalletContext.Provider value={value}>
            {children}
        </WalletContext.Provider>
    );
};
