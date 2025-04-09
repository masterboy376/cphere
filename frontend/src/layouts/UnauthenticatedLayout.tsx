import { Outlet } from 'react-router-dom'
import { Footer } from '../components/common/Footer'
import { useEffect } from 'react'
import authBackendApiService from '../services/auth/AuthBackendApiService'
import { useAuthentication } from '../contexts/AuthenticationContext'

export const UnauthenticatedLayout = () => {
  const { setUserId } = useAuthentication()
  useEffect(() => {
    const checkUserAuthentication = async () => {
      try {
        const status = await authBackendApiService.authStatus();
        if (status.user_id) {
          setUserId(status.user_id);
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