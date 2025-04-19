// SearchPage.tsx
import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { UserAvatar } from '../../components/chat/UserAvatar'
import userBackendApiService, { UserSearchQuery } from '../../services/user/UserBackendApiService'
import chatBackendApiService, { ChatsCreatePayload } from '../../services/chat/ChatBackendApiService'
import { Loader } from '../../components/common/Loader'
import { useAuthentication } from '../../contexts/AuthenticationContext'

interface UserResult {
  id: string
  username: string
}

export const SearchPage = () => {
  const navigate = useNavigate()
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<UserResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const { authState } = useAuthentication()

  const searchUsers = async () => {
    if (query.length < 3) return

    setIsLoading(true)
    try {
      // if (!userId) {
      //   console.error('User ID is not available')
      //   return
      // }
      // let isOnline = await userBackendApiService.isOnline(userId)
      // console.log('User is online:', isOnline)
      const payload: UserSearchQuery = { 'q': query }
      const data = await userBackendApiService.search(payload)
      const filteredResults = data.filter((user: UserResult) => user.id !== authState.userId)
      setResults(filteredResults)
    } catch (error) {
      console.error('Search failed:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleStartChat = async (userId: string) => {
    try {
      const payload: ChatsCreatePayload = { participant_id: userId }
      const data = await chatBackendApiService.create(payload)

      console.log(data)
      
      const chatId = await data.id
      navigate(`/chats/${chatId}`)
    } catch (error) {
      console.error('Failed to start chat:', error)
    }
  }
  
  useEffect(() => {
    // Debounced search
    const debounceTimer = setTimeout(searchUsers, 300)
    return () => clearTimeout(debounceTimer)
  }, [query])

  return (
    <div className="flex-1 flex flex-col">
      {/* Search Box */}
      <div className="p-4 border-b border-background-lite">
        <div className="relative">
          <MagnifyingGlassIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
          <input
            type="text"
            placeholder="Search users..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-background-lite"
          />
        </div>
      </div>

      {/* Results */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <Loader message='Searching...'/>
        ) : results.length > 0 ? (
          results.map((user) => (
            <button
              key={user.id}
              onClick={() => handleStartChat(user.id)}
              className="w-full p-4 flex items-center gap-4 hover:bg-background-paper transition-colors"
            >
              <div className="h-10 w-10 bg-primary rounded-full flex items-center justify-center text-white">
                <UserAvatar username={user.username} />
              </div>
              <span className="text-text-primary">{user.username}</span>
            </button>
          ))
        ) : query.length >= 3 ? (
          <div className="p-8 text-center text-text-secondary">No users found</div>
        ) : (
          <div className="p-8 text-center text-text-secondary">
            {query.length > 0 ? 'Type at least 3 characters' : 'Search for users to start chatting'}
          </div>
        )}
      </div>
    </div>
  )
}