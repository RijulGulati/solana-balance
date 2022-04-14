import {
  Alert,
  AlertIcon,
  AlertTitle,
  Box,
  Button,
  Input,
  InputGroup,
  InputRightElement,
  Select,
  Stack,
  Text,
} from '@chakra-ui/react';
import React from 'react';
import './App.css';

const API_URL = process.env.REACT_APP_API_URL;

interface HttpResponse<T> {
  status: number;
  data: T;
}

interface SearchFormProps {}

interface SearchResponse {
  lamports: number;
  sol: number;
  error?: string;
}

interface SearchFormState {
  pubkey: string;
  cluster: number;
  result: SearchResponse | undefined;
  showLoader: boolean;
}

interface SearchResultProps {
  status: 'success' | 'error' | undefined;
  lamports?: number;
  sol?: number;
  error?: string;
}

interface SearchResultState {
  status: 'success' | 'error' | undefined;
}

class App extends React.Component {
  render(): React.ReactNode {
    return (
      <div className='container'>
        <h1>
          <span className='color'>Solana</span> Blockchain
        </h1>

        <Box w={'60%'} marginTop={10}>
          <Stack spacing={4}>
            <Text fontSize={18} fontStyle={'italic'}>
              Get account balance from Mainnet/Testnet/Devnet cluster.
            </Text>
            <SearchForm />
          </Stack>
        </Box>
      </div>
    );
  }
}

class SearchForm extends React.Component<SearchFormProps, SearchFormState> {
  constructor(props: SearchFormProps) {
    super(props);
    this.state = {
      cluster: 1,
      pubkey: '',
      result: undefined,
      showLoader: false,
    };
  }

  onSubmit = (event: React.MouseEvent<HTMLElement>) => {
    debugger;
    event.preventDefault();
    this.setState({
      showLoader: true,
    });
    fetch(
      `${API_URL}/balance?cluster=${this.state.cluster}&pubkey=${this.state.pubkey}`
    )
      .then((res) => {
        this.setState({
          showLoader: false,
        });
        if (res.status === 200 || res.status === 400) {
          res
            .json()
            .then((res: HttpResponse<SearchResponse>) => {
              this.setState({
                result: res.data,
              });
            })
            .catch((err) => {
              console.log('json parse error: ', err);
            });
        } else {
          console.log('non-200 http response: ', res);
        }
      })
      .catch((err) => {
        this.setState({
          showLoader: false,
        });
        this.setState({
          result: {
            error: 'Network error!',
            lamports: 0,
            sol: 0,
          },
        });
      });
  };

  render() {
    return (
      <Box>
        <Stack spacing={6}>
          <form>
            <InputGroup size={'lg'}>
              <Input
                name='pubkey'
                pr={'8em'}
                variant='outline'
                placeholder='Account pubkey'
                size={'lg'}
                focusBorderColor='teal.500'
                borderRadius={0}
                onChange={(e) => this.setState({ pubkey: e.target.value })}
              />
              <InputRightElement width='8em'>
                <Select
                  name='cluster'
                  focusBorderColor='teal.500'
                  size={'lg'}
                  variant={'outline'}
                  borderRadius={0}
                  value={this.state.cluster}
                  onChange={(e) =>
                    this.setState({ cluster: parseInt(e.target.value) })
                  }
                >
                  <option value={1}>Mainnet</option>
                  <option value={2}>Testnet</option>
                  <option value={3}>Devnet</option>
                </Select>
              </InputRightElement>
            </InputGroup>

            <Button
              type='submit'
              marginTop={5}
              alignSelf={'center'}
              w={'36'}
              colorScheme={'teal'}
              onClick={this.onSubmit}
              isLoading={this.state.showLoader ? true : false}
            >
              Submit
            </Button>
          </form>

          {this.state.result ? (
            <SearchResult
              status={
                this.state.result
                  ? this.state.result.error
                    ? 'error'
                    : 'success'
                  : undefined
              }
              lamports={this.state.result?.lamports}
              sol={this.state.result?.sol}
              error={this.state.result?.error}
            />
          ) : null}
        </Stack>
      </Box>
    );
  }
}

class SearchResult extends React.Component<
  SearchResultProps,
  SearchResultState
> {
  render() {
    if (this.props.status) {
      if (this.props.status === 'success') {
        return (
          <Alert status='success'>
            <AlertIcon />
            <AlertTitle>
              {this.props.lamports} Lamports ({this.props.sol} SOL)
            </AlertTitle>
          </Alert>
        );
      } else {
        return (
          <Alert status='error'>
            <AlertIcon />
            <AlertTitle>{this.props.error}</AlertTitle>
          </Alert>
        );
      }
    } else {
      return null;
    }
  }
}

export default App;
