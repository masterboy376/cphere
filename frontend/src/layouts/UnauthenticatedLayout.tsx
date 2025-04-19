import { Outlet } from 'react-router-dom'
import { Footer } from '../components/common/Footer'
import { useEffect } from 'react'
import authBackendApiService from '../services/auth/AuthBackendApiService'
import { useAuthentication } from '../contexts/AuthenticationContext'

export const UnauthenticatedLayout = () => {
  const { setAuthState } = useAuthentication()
  useEffect(() => {
    const checkUserAuthentication = async () => {
      try {
        const status = await authBackendApiService.authStatus();
        setAuthState({userId: status.user_id, username: status.username});
        if (status.user_id) {
          window.location.href = '/chats';
        }
      } catch (error) {
        console.error("Error checking authentication:", error);
      }
    };
    checkUserAuthentication();
  }, []);

  return (
    <>
      <Outlet />
      <Footer />
    </>
  )
}