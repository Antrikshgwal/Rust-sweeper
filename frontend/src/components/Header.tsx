"use client";
import { useWallet } from '@/contexts/WalletContext';

export const Header = () => {
    const { address, connect, disconnect } = useWallet();

    return (
        <header className="flex justify-between items-center p-4 bg-gray-800 text-white">
            <h1 className="text-xl font-bold">Dust Sweeper</h1>
            <div>
                {address ? (
                    <div className="flex items-center">
                        <p className="mr-4">{`${address.substring(0, 6)}...${address.substring(address.length - 4)}`}</p>
                        <button
                            onClick={disconnect}
                            className="px-4 py-2 bg-red-500 rounded hover:bg-red-600"
                        >
                            Disconnect
                        </button>
                    </div>
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
