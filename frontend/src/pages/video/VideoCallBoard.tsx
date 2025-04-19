import { useState, useEffect, useRef } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { VideoCameraSlashIcon, XMarkIcon } from '@heroicons/react/24/outline'
import { VideoCallResponse } from '../../types/WsMessageTypes'
import wsService from '../../services/ws/WsService'


export const VideoCallBoard = () => {
  const { particitantId } = useParams()
  const navigate = useNavigate()
  const [callStatus, setCallStatus] = useState('calling')
  const [remoteStream, setRemoteStream] = useState<MediaStream | null>(null)
  const localVideoRef = useRef<HTMLVideoElement>(null)
  const remoteVideoRef = useRef<HTMLVideoElement>(null)
  const [peerConnection, setPeerConnection] = useState<RTCPeerConnection | null>(null)

  // Initialize local media
  useEffect(() => {
    const videoCallAceptedListener = (message: VideoCallResponse) => {
      if (message.type === 'video_call_accepted') {
        setCallStatus('connected')
      }
    }

    const videoCallDeclinedListener = (message: VideoCallResponse) => {
      if (message.type === 'video_call_declined') {
        setCallStatus('declined')
        navigate(-1)
      }
    }

    const initLocalMedia = async () => {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({
          video: true,
          audio: true
        })
        
        if (localVideoRef.current) {
          localVideoRef.current.srcObject = stream
        }

        // TODO: Create peer connection and send offer
        const pc = new RTCPeerConnection()
        setPeerConnection(pc)

        // Add local stream to connection
        stream.getTracks().forEach(track => pc.addTrack(track, stream))

        // Setup remote stream
        pc.ontrack = (event) => {
          setRemoteStream(event.streams[0])
        }

        // Create and send offer
        const offer = await pc.createOffer()
        await pc.setLocalDescription(offer)
        
        // Simulate sending offer to remote peer (replace with actual signaling)
        setTimeout(() => {
          // Simulate receiving answer
          pc.setRemoteDescription({
            type: 'answer',
            sdp: 'simulated-answer-sdp'
          })
          setCallStatus('connected')
        }, 3000)

      } catch (error) {
        console.error('Error accessing media devices:', error)
        setCallStatus('failed')
      }
    }

    initLocalMedia()
    wsService.addEventListener('video_call_accepted', videoCallAceptedListener)
    wsService.addEventListener('video_call_declined', videoCallDeclinedListener)

    return () => {
      peerConnection?.close()
      localVideoRef.current?.srcObject?.getTracks().forEach(track => track.stop())
      wsService.removeEventListener('video_call_accepted', videoCallAceptedListener)
      wsService.removeEventListener('video_call_declined', videoCallDeclinedListener)
    }
  }, [])

  const handleEndCall = () => {
    peerConnection?.close()
    localVideoRef.current?.srcObject?.getTracks().forEach(track => track.stop())
    navigate(-1)
  }

  return (
    <div className="h-screen w-full bg-background flex flex-col">
      {/* Video Containers */}
      <div className="flex-1 grid grid-cols-1 md:grid-cols-2 gap-4 p-4">
        {/* Local Video */}
        <div className="relative bg-background-paper rounded-lg overflow-hidden">
          <video
            ref={localVideoRef}
            autoPlay
            muted
            className="h-full w-full object-cover"
          />
          <div className="absolute bottom-2 left-2 text-sm text-white bg-black/50 px-2 py-1 rounded">
            You
          </div>
        </div>

        {/* Remote Video */}
        <div className="relative bg-background-paper rounded-lg overflow-hidden">
          {remoteStream ? (
            <video
              ref={remoteVideoRef}
              autoPlay
              className="h-full w-full object-cover"
              onCanPlay={() => remoteVideoRef.current?.play()}
            />
          ) : (
            <div className="h-full w-full flex items-center justify-center">
              <div className="text-center">
                <VideoCameraSlashIcon className="h-12 w-12 text-gray-500 mx-auto mb-2" />
                <p className="text-gray-400">
                  {callStatus === 'calling' ? 'Calling...' : 'Waiting for answer'}
                </p>
              </div>
            </div>
          )}
          {remoteStream && (
            <div className="absolute bottom-2 left-2 text-sm text-white bg-black/50 px-2 py-1 rounded">
              Remote User
            </div>
          )}
        </div>
      </div>

      {/* Controls */}
      <div className="p-4 flex justify-center bg-background-paper">
        <button
          onClick={handleEndCall}
          className="flex items-center gap-2 px-6 py-3 bg-red-600 hover:bg-red-700 text-white rounded-full transition-colors"
        >
          <XMarkIcon className="h-6 w-6" />
          End Call
        </button>
      </div>
    </div>
  )
}