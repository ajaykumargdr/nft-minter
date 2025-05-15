import React from 'react';

const CLIENT_ID = import.meta.env.VITE_CLIENT_ID;

const App = () => {

  const oauthSignIn = () => {

    var oauth2Endpoint = 'https://accounts.google.com/o/oauth2/v2/auth';

    // Create <form> element to submit parameters to OAuth 2.0 endpoint.
    var form = document.createElement('form');
    form.setAttribute('method', 'GET'); // Send as a GET request.
    form.setAttribute('action', oauth2Endpoint);

    // Parameters to pass to OAuth 2.0 endpoint.
    var params = {
      'client_id': CLIENT_ID,
      'redirect_uri': 'http://localhost:8080/auth/google',
      'response_type': 'code',
      'scope': 'email profile'
    };

    // Add form parameters as hidden input values.
    for (var p in params) {
      var input = document.createElement('input');
      input.setAttribute('type', 'hidden');
      input.setAttribute('name', p);
      input.setAttribute('value', params[p]);
      form.appendChild(input);
    }

    // Add form to page and submit it to open the OAuth 2.0 endpoint.
    document.body.appendChild(form);
    form.submit();
  }

  return (
    <div>
      <button onClick={oauthSignIn}> <h2> {'->'} Sign In </h2></button>
    </div>
  );
};

export default App;