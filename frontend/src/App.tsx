import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'

import { ChatBoard } from './pages/chat/ChatBoard'
import { ChatsPage } from './pages/chat/ChatsPage'
import { ForgotPasswordPage } from './pages/auth/ForgetPasswordPage'
import { LoginPage } from './pages/auth/LoginPage'
import { RegisterPage } from './pages/auth/RegisterPage'
import { ResetPasswordPage } from './pages/auth/ResetPasswordPage'
import { SearchPage } from './pages/search/SerachPage'
import { VideoCallBoard } from './pages/video/VideoCallBoard'

import { UnauthenticatedLayout } from './layouts/UnauthenticatedLayout'
import { AuthenticatedLayout } from './layouts/AuthenticatedLayout'

import { AuthenticationProvider } from './contexts/AuthenticationContext'
import { ChatProvider } from './contexts/ChatContext'
import { NavigationProvider } from './contexts/NavigationContext'

function App() {

  return (
    <Router>
      <AuthenticationProvider>
        <ChatProvider>
          <NavigationProvider>
            <Routes>
              {/* Authenticated Routes */}
              <Route element={<AuthenticatedLayout />}>
                <Route path="/search" element={<SearchPage />} />
                <Route path="/chats" element={<ChatsPage />} />
                <Route path="/chats/:chatId" element={<ChatBoard />} />
              </Route>

              {/* Unauthenticated Routes */}
              <Route element={<UnauthenticatedLayout />}>
                <Route index element={<LoginPage />} />
                <Route path="/login" element={<LoginPage />} />
                <Route path="/register" element={<RegisterPage />} />
                <Route path="/forgot-password" element={<ForgotPasswordPage />} />
                <Route path="/reset-password/:token" element={<ResetPasswordPage />} />
              </Route>

              {/* Video Call Route */}
              <Route path="/video-call/:videoId" element={<VideoCallBoard />} />
            </Routes>
          </NavigationProvider>
        </ChatProvider>
      </AuthenticationProvider>
    </Router>
  )
}

export default App