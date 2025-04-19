import { createContext, useContext, useState, Dispatch, SetStateAction } from 'react'

type AuthenticationContextType = {
  authState: { userId: string | null, username: string | null } 
  setAuthState: Dispatch<SetStateAction<{ userId: string | null, username: string | null } >>
}

const AuthenticationContext = createContext<AuthenticationContextType>({
    authState: { userId: null, username: null },
    setAuthState: () => {}
})

export const AuthenticationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [authState, setAuthState] = useState<{ userId: string | null, username: string | null }>({ userId: null, username: null });
 
  return (
    <AuthenticationContext.Provider value={{ authState, setAuthState }}>
      {children}
    </AuthenticationContext.Provider>
  )
}

export const useAuthentication = () => useContext(AuthenticationContext)