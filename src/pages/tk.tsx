import { useState } from 'react'
import Editor from '../components/Editor'

export default function TkPage () {
    const [editorText, setEditorText] = useState<string>('<h2>Test editor</h2>')

    return (
        <div className='mx-20 my-10'>
            <div>
                <h2>Editor ({editorText})</h2>
                <Editor value={editorText} onChange={(e:string) => setEditorText(e)} />
            </div>
        </div>
    )
}