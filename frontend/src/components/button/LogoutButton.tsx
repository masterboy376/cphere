import React from 'react'
import { ArrowRightStartOnRectangleIcon } from '@heroicons/react/24/solid'
import authBackendApiService from '../../services/auth/AuthBackendApiService.ts' // Adjust the path as needed
import { useNavigate } from 'react-router-dom'
import wsService from '../../services/ws/WsService.ts'
import { useAuthentication } from '../../contexts/AuthenticationContext.tsx'


interface LogoutButtonProps { }

const LogoutButton: React.FC<LogoutButtonProps> = () => {
    const navigate = useNavigate()
    const { setAuthState } = useAuthentication()

    const handleLogout = async () => {
        try {
            await authBackendApiService.logout()
            wsService.sendMessage({ type: 'logout' });
            setAuthState({userId: null, username: null})
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
