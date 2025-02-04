import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'

import { ChatBoard } from './features/chat/ChatBoard'
import { ChatsPage } from './features/chat/ChatsPage'
import { ForgotPasswordPage } from './features/auth/ForgetPasswordPage'
import { LoginPage } from './features/auth/LoginPage'
import { RegisterPage } from './features/auth/RegisterPage'
import { ResetPasswordPage } from './features/auth/ResetPasswordPage'
import { SearchPage } from './features/search/SerachPage'
import { VideoCallBoard } from './features/video/VideoCallBoard'

import { UnauthenticatedLayout } from './layouts/UnauthenticatedLayout'
import { AuthenticatedLayout } from './layouts/AuthenticatedLayout'

import { NavigationProvider } from './contexts/NavigationContext'

function App() {

  return (
    <Router>
      <NavigationProvider>
        <Routes>
          {/* Authenticated Routes */}
          <Route element={<AuthenticatedLayout />}>
              <Route path="/search" element={<SearchPage />} />
              <Route path="/" element={<ChatsPage />} />
              <Route path="/chats/:chatId" element={<ChatBoard />} />
          </Route>

          {/* Unauthenticated Routes */}
          <Route element={<UnauthenticatedLayout />}>
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />
            <Route path="/forgot-password" element={<ForgotPasswordPage />} />
            <Route path="/reset-password/:token" element={<ResetPasswordPage />} />
          </Route>

          {/* Video Call Route */}
          <Route path="/video-call/:videoId" element={<VideoCallBoard />} />
        </Routes>
      </NavigationProvider>
    </Router>
  )
}

export default App