import React from 'react'
import { ArrowRightStartOnRectangleIcon } from '@heroicons/react/24/solid'
import authBackendApiService from '../../services/auth/AuthBackendApiService.ts' // Adjust the path as needed
import { useNavigate } from 'react-router-dom'


interface LogoutButtonProps { }

const LogoutButton: React.FC<LogoutButtonProps> = () => {
    const navigate = useNavigate()
    const handleLogout = async () => {
        try {
            await authBackendApiService.logout()
            navigate('/login')
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
            <ArrowRightStartOnRectangleIcon className="h-6 w-6" />
        </button>
    )
}

export default LogoutButton
