import { createContext, useContext, useState, Dispatch, SetStateAction } from 'react'

type AuthenticationContextType = {
  userId: null | string
  setUserId: Dispatch<SetStateAction<string | null>>
}

const AuthenticationContext = createContext<AuthenticationContextType>({
    userId: null,
    setUserId: () => {}
})

export const AuthenticationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [userId, setUserId] = useState<null | string>(null)

  return (
    <AuthenticationContext.Provider value={{ userId, setUserId }}>
      {children}
    </AuthenticationContext.Provider>
  )
}

export const useAuthentication = () => useContext(AuthenticationContext)