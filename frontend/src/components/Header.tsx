"use client";
import { useWallet } from '@/contexts/WalletContext';

export const Header = () => {
    const { address, connect, disconnect } = useWallet();

    return (
        <header className="flex justify-between items-center p-4 bg-gray-800 text-white">
            <h1 className="text-xl font-bold">Dust Sweeper</h1>
            <div className="flex items-center gap-3">
                {address ? (
                    <>
                        <span className="px-2 py-1 text-xs bg-green-900 text-green-300 rounded">
                            Sepolia
                        </span>
                        <p className="text-sm text-gray-300">{`${address.substring(0, 6)}...${address.substring(address.length - 4)}`}</p>
                        <button
                            onClick={disconnect}
                            className="px-4 py-2 bg-red-500 rounded hover:bg-red-600 text-sm"
                        >
                            Disconnect
                        </button>
                    </>
                ) : (
                    <button
                        onClick={connect}
                        className="px-4 py-2 bg-blue-500 rounded hover:bg-blue-600"
                    >
                        Connect Wallet
                    </button>
                )}
            </div>
        </header>
    );
};
