// SearchPage.tsx
import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline'

interface UserResult {
  id: string
  username: string
  avatar?: string
}

export const SearchPage = () => {
  const navigate = useNavigate()
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<UserResult[]>([])
  const [isLoading, setIsLoading] = useState(false)

  // Debounced search
  useEffect(() => {
    const searchUsers = async () => {
      if (query.length < 3) return
      
      setIsLoading(true)
      try {
        const response = await fetch(`/api/users/search?q=${encodeURIComponent(query)}`)
        const data = await response.json()
        setResults(data)
      } catch (error) {
        console.error('Search failed:', error)
      } finally {
        setIsLoading(false)
      }
    }

    const debounceTimer = setTimeout(searchUsers, 300)
    return () => clearTimeout(debounceTimer)
  }, [query])

  const startChat = async (userId: string) => {
    try {
      const response = await fetch('/api/chats', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ participantId: userId })
      })
      
      const { chatId } = await response.json()
      navigate(`/chats/${chatId}`)
    } catch (error) {
      console.error('Failed to start chat:', error)
    }
  }

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
          <div className="p-8 text-center text-text-secondary">Searching...</div>
        ) : results.length > 0 ? (
          results.map((user) => (
            <button
              key={user.id}
              onClick={() => startChat(user.id)}
              className="w-full p-4 flex items-center gap-4 hover:bg-background-paper transition-colors"
            >
              <div className="h-10 w-10 bg-primary rounded-full flex items-center justify-center text-white">
                {user.avatar ? (
                  <img src={user.avatar} alt={user.username} className="rounded-full" />
                ) : (
                  user.username[0].toUpperCase()
                )}
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