"use client";
import { useState, useEffect, useCallback } from 'react';
import { ethers } from 'ethers';
import { WalletContext, WalletState } from '@/contexts/WalletContext';

const SEPOLIA_CHAIN_ID = '0xaa36a7'; // 11155111

interface WalletProviderProps {
    children: React.ReactNode;
}

async function switchToSepolia() {
    try {
        await window.ethereum.request({
            method: 'wallet_switchEthereumChain',
            params: [{ chainId: SEPOLIA_CHAIN_ID }],
        });
    } catch (switchError: any) {
        // Chain not added — add it
        if (switchError.code === 4902) {
            await window.ethereum.request({
                method: 'wallet_addEthereumChain',
                params: [{
                    chainId: SEPOLIA_CHAIN_ID,
                    chainName: 'Sepolia Testnet',
                    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
                    rpcUrls: ['https://rpc.sepolia.org'],
                    blockExplorerUrls: ['https://sepolia.etherscan.io'],
                }],
            });
        } else {
            throw switchError;
        }
    }
}

export const WalletProvider = ({ children }: WalletProviderProps) => {
    const [provider, setProvider] = useState<ethers.BrowserProvider | null>(null);
    const [signer, setSigner] = useState<ethers.JsonRpcSigner | null>(null);
    const [address, setAddress] = useState<string | null>(null);

    const connect = useCallback(async () => {
        if (typeof window.ethereum === 'undefined') {
            console.error("MetaMask is not installed");
            return;
        }

        try {
            await switchToSepolia();
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

            const handleChainChanged = () => {
                // Reconnect to ensure we're still on Sepolia
                connect();
            };

            window.ethereum.on('accountsChanged', handleAccountsChanged);
            window.ethereum.on('chainChanged', handleChainChanged);

            return () => {
                if (window.ethereum.removeListener) {
                    window.ethereum.removeListener('accountsChanged', handleAccountsChanged);
                    window.ethereum.removeListener('chainChanged', handleChainChanged);
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
