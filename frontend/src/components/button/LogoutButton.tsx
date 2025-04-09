import React from 'react'
import { ArrowRightOnRectangleIcon } from '@heroicons/react/24/solid'
import authBackendApiService from '../../services/auth/AuthBackendApiService.ts' // Adjust the path as needed


interface LogoutButtonProps { }

const LogoutButton: React.FC<LogoutButtonProps> = () => {
    const handleLogout = async () => {
        try {
            await authBackendApiService.logout()
        } catch (error) {
            console.error('Logout failed', error)
        }
    }

    return (
        <button
            onClick={handleLogout}
            className="text-red-500 hover:bg-background-lite p-2 transition-all duration-300 ease-in-out rounded-full"
            type="button"
        >
            <ArrowRightOnRectangleIcon className="h-6 w-6" />
        </button>
    )
}

export default LogoutButton
