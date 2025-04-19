import { Outlet } from 'react-router-dom'
import { SideNavigationBar } from '../components/navigation/SideNavigationBar'
import { TopNavigationBar } from '../components/navigation/TopNavigationBar'
import { MainContentWrapper } from '../wrappers/MainContentWrapper'
import { useEffect } from 'react'
import authBackendApiService from '../services/auth/AuthBackendApiService'
import { useAuthentication } from '../contexts/AuthenticationContext'
import wsService from '../services/ws/WsService'

export const AuthenticatedLayout = () => {
  const { setAuthState } = useAuthentication()
  useEffect(() => {
    const checkUserAuthentication = async () => {
      try {
        const status = await authBackendApiService.authStatus();
        setAuthState({userId: status.user_id, username: status.username});
        if (!status.user_id) {
          window.location.href = '/login';
        }
        else {
          wsService.connect();
        }
      } catch (error) {
        console.error("Error checking authentication:", error);
      }
    };
    checkUserAuthentication();
  }, []);
  return (
    <div className="h-screen flex flex-col">
      <TopNavigationBar />
      <div className="flex-1 flex flex-col min-h-0">
        <SideNavigationBar />
        {/* Main Content Area */}
        <MainContentWrapper>
          <Outlet />
        </MainContentWrapper>
      </div>
    </div>
  )
}