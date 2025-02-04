import { format, formatDistanceToNow } from 'date-fns'

interface MessageCardProps {
  id: string
  text: string
  sender: string
  timestamp: Date
}

export const MessageCard = ({ text, sender, timestamp }: MessageCardProps) => {
  const isMe = sender === 'me'

  return (
    <div className={`flex ${isMe ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`max-w-[75%] rounded-lg p-4 ${
          isMe 
            ? 'bg-primary text-white rounded-br-none' 
            : 'bg-background-paper text-text-primary rounded-bl-none'
        }`}
      >
        <p className="break-words">{text}</p>
        <div className={`mt-2 text-xs ${isMe ? 'text-primary-100' : 'text-text-secondary'}`}>
          {format(timestamp, 'HH:mm')}
          <span className="mx-2">•</span>
          {formatDistanceToNow(timestamp, { addSuffix: true })}
        </div>
      </div>
    </div>
  )
}