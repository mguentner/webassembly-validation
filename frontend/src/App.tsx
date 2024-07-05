import { useEffect, useState } from 'react'
import './App.css'
import { validate_create_host_params } from "wasm_validation";

type Error = {
  type: "Error";
  error: string
  path: string | null;
}

type Success = {
  type: "Success";
  validated: object;
}

type Result = Success | Error;

function App() {
  const [result, setResult] = useState<Result | undefined>(undefined);
  const [value, setValue] = useState(`{
    "hostname": "foobar",
    "ipv4": "192.168.1.1337"
  }`);
  const [requestResult, setRequestResult] = useState<string | undefined>(undefined);
  
  const onTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setValue(event.target.value);
  }
  
  useEffect(() => {
    const res = validate_create_host_params(value) as Result;
    setResult(res);
    console.log(res);
  }, [value])

  const sendRequest = () => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: value
    };

    fetch('http://127.0.0.1:3000/hosts', requestOptions)
     .then(response => response.text())
     .then(data => setRequestResult(data))
     .catch(error => setRequestResult(JSON.stringify(error)));
  }
  
  return (
    <>
      <h1>wasm-bindgen serde parsing demo</h1>
      <div className='flexarea'>
        <textarea id="textarea" onChange={onTextChange} value={value}></textarea>
        { result !== undefined && result.type === "Success" ? <div className="success">{JSON.stringify(result.validated)}</div> : null }
        { result !== undefined && result.type === "Error" ? <div className="error">Error: {result.error} Path: {result.path}</div> : null }
      </div>
      { requestResult !== undefined && <div><h2>The Server says</h2><pre>{requestResult}</pre></div>}
      
      { result !== undefined && result.type === "Success" ? <div><h2>🎆 Your data is correct. You may now send the request</h2><button onClick={sendRequest}>Send</button></div> : null}
    </>
  )
}

export default App
