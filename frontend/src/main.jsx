import React, { useState, useRef, useEffect } from 'react'
import ReactDOM from 'react-dom/client'
import '/index.css'
import { backend } from 'declarations/backend'
import {
  ArrowUp,
  Avatar,
  Button,
  ChatBubble,
  ChatBubbleMessage,
  ChatBubbleActionWrapper,
  ChatInput,
  Copy,
  ChatBubbleAction
} from './components'
import botImg from '/bot.svg'

const ChatActions = [
  {
    icon: Copy,
    type: 'copy'
  }
]

const App = () => {
  const [chat, setChat] = useState([])
  const [inputValue, setInputValue] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isStarted, setIsStarted] = useState(false)
  const [toast, setToast] = useState(undefined)
  const chatBoxRef = useRef(null)

  const showToast = message => {
    setToast(message)
    setTimeout(() => {
      setToast(undefined)
    }, 2000)
  }

  const replaceLatestChatWithResponse = response => {
    setChat(prevChat => {
      const newChat = [...prevChat]
      newChat.pop()
      newChat.push({ role: { assistant: null }, content: response })
      return newChat
    })
  }

  const askAgent = async (input, onSuccess = replaceLatestChatWithResponse) => {
    try {
      const response = await backend.prompt(input)
      onSuccess(response)
    } catch (e) {
      console.log(e)
      const eStr = String(e)
      const match = eStr.match(/(SysTransient|CanisterReject), \\+"([^\\"]+)/)
      if (match) {
        alert(match[2])
      }
      setChat(prevChat => {
        const newChat = [...prevChat]
        if (newChat.length > 0 && newChat[newChat.length - 1].loading) newChat.pop()
        return newChat
      })
    } finally {
      setIsLoading(false)
    }
  }

  const onKeyDown = e => {
    if (e.key === 'Enter' && !e.shiftKey) {
      handleSubmit(e)
    }
  }

  const handleSubmit = e => {
    e.preventDefault()
    if (!inputValue.trim() || isLoading) return

    const userMessage = {
      role: { user: null },
      content: inputValue
    }
    const thinkingMessage = {
      role: { assistant: null },
      content: 'Thinking',
      loading: true
    }
    setChat(prevChat => [...prevChat, userMessage, thinkingMessage])
    setInputValue('')
    setIsLoading(true)
    askAgent(inputValue)
  }

  const handleCmd = cmd => {
    const thinkingMessage = {
      role: { assistant: null },
      content: 'Thinking',
      loading: true
    }
    if (cmd === '/start') {
      setChat(prevChat => [...prevChat, thinkingMessage])
      setIsLoading(true)
      askAgent(cmd, response => {
        setChat([{ role: { assistant: null }, content: response }])
        setIsStarted(true)
      })
    } else if (cmd === '/about') {
      setChat(prevChat => [...prevChat, thinkingMessage])
      setIsLoading(true)
      askAgent(cmd)
    }
  }

  const handleActionClick = async (action, messageIndex) => {
    console.log('Action clicked:', action, 'Message index:', messageIndex)

    if (action === 'copy') {
      const message = chat[messageIndex]
      if (message) {
        try {
          await navigator.clipboard.writeText(message.content)
          showToast('Copied to clipboard!')
        } catch (err) {
          console.error('Failed to copy text to clipboard:', err)
          alert('Failed to copy text. Please try again.')
        }
      }
    }
  }

  useEffect(() => {
    if (chatBoxRef.current) {
      chatBoxRef.current.scrollTop = chatBoxRef.current.scrollHeight
    }
  }, [chat])

  return (
    <div className='flex flex-col h-full w-full overflow-hidden'>
      {toast && (
        <div className='fixed bottom-40 left-1/2 transform -translate-x-1/2 bg-secondary text-white px-4 py-2 rounded shadow-lg z-50'>
          {toast}
        </div>
      )}
      <div className='flex flex-col relative w-full h-full'>
        <div className='draggable no-draggable-children sticky top-0 p-3 flex items-center justify-center z-10 pointer-events-none select-none'>
          <div className='flex items-center gap-0 overflow-hidden'>
            {!isStarted && chat.length === 0 ? (
              <h1 className='md:text-5xl text-4xl font-bold [@media(max-height:600px)]:my-4 md:my-40 my-12'>
                dHisStoryGame.AI
              </h1>
            ) : (
              <h1 className='text-3xl font-bold'>dHisStoryGame.AI</h1>
            )}
          </div>
        </div>
        <div className='flex flex-col w-full h-full p-4 overflow-y-auto'>
          <div className='flex flex-col gap-6'>
            {!isStarted && chat.length === 0 && (
              <div
                className='flex rounded-full h-56 w-56 self-center my-12'
                style={{ backgroundImage: `url(${botImg})`, backgroundSize: 'cover' }}
              ></div>
            )}
            {chat.map((message, index) => {
              const variant = 'user' in message.role ? 'sent' : 'received'
              return (
                <ChatBubble key={index} variant={variant}>
                  <Avatar variant={variant} />
                  <ChatBubbleMessage variant={variant} isLoading={message.loading}>
                    {message.content}
                  </ChatBubbleMessage>
                  <ChatBubbleActionWrapper>
                    {ChatActions.map(({ icon: Icon, type }) => (
                      <ChatBubbleAction
                        className='size-7'
                        key={type}
                        icon={<Icon className='size-4' />}
                        onClick={() => handleActionClick(type, index)}
                      />
                    ))}
                  </ChatBubbleActionWrapper>
                </ChatBubble>
              )
            })}
          </div>
        </div>
        <div className='w-full px-4 pb-4 relative'>
          <div className='mx-auto flex flex-1 gap-4 md:gap-5 lg:gap-6 md:max-w-3xl lg:max-w-[40rem] xl:max-w-[48rem]'>
            <div className='relative z-[1] flex max-w-full flex-1 flex-col h-full'>
              {!isStarted && (
                <div className='flex rounded-lg bg-muted/70 min-h-16 p-3 w-full justify-center gap-10'>
                  <Button size='xl' onClick={() => handleCmd('/start')} disabled={isLoading}>
                    Start
                  </Button>
                  <Button
                    size='xl'
                    variant='secondary'
                    onClick={() => handleCmd('/about')}
                    disabled={isLoading}
                    className='bg-slate-700'
                  >
                    About
                  </Button>
                </div>
              )}
              {isStarted && (
                <form
                  className='relative rounded-lg border bg-muted focus-within:ring-1 focus-within:ring-ring py-3 pl-3'
                  onSubmit={handleSubmit}
                >
                  <ChatInput
                    onKeyDown={onKeyDown}
                    onChange={e => setInputValue(e.target.value)}
                    value={inputValue}
                    placeholder='Write your story'
                    className='bg-transparent'
                  />
                  <div className='flex p-3 pt-0'>
                    <Button type='submit' disabled={isLoading || !inputValue} className='ml-auto gap-1.5' size='sm'>
                      Send
                      <ArrowUp />
                    </Button>
                  </div>
                </form>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
