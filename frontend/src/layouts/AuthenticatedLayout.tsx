import { Outlet } from 'react-router-dom'

import { SideNavigationBar } from '../components/navigation/SideNavigationBar'
import { TopNavigationBar } from '../components/navigation/TopNavigationBar'

import { MainContentWrapper } from '../wrappers/MainContentWrapper'

export const AuthenticatedLayout = () => {
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