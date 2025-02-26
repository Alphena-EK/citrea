import unittest
from hexbytes import HexBytes
from web3 import Web3

class TestWeb3(unittest.TestCase):
    def setUp(self):
        self.web3 = Web3(Web3.HTTPProvider('http://127.0.0.1:12346'))
        transaction = {
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'to': "0x0000000000000000000000000000000000000000",
            'value': 1000000000,
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'gas': 200000,
            'gasPrice': self.web3.eth.gas_price,
        }
        signed_tx = self.web3.eth.account.sign_transaction(transaction, "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        self.first_tx_hash = self.web3.eth.send_raw_transaction(signed_tx.raw_transaction)
        self.web3.eth.wait_for_transaction_receipt(self.first_tx_hash)

    def test_connection(self):
        self.assertEqual(self.web3.is_connected(), True)
    
    def test_max_priority_fee(self):
        max_priority_fee = self.web3.eth.max_priority_fee
        self.assertGreater(max_priority_fee, 0)

    def test_gas_price(self):
        gas_price = self.web3.eth.gas_price
        self.assertGreater(gas_price, 0)

    def test_chain_id(self):
        chain_id = self.web3.eth.chain_id
        self.assertEqual(chain_id, 5655)

    def test_block_number(self):
        block_number = self.web3.eth.get_block_number()
        self.assertIsInstance(self.web3.eth.get_block_number(), int)
        self.assertGreater(block_number, 0)

    def test_get_balance(self):
        balance = self.web3.eth.get_balance('0x3100000000000000000000000000000000000002')
        self.assertGreaterEqual(balance, (21 * 10 ** 6) * 10 ** 18)

    def test_get_storage_at(self):
        slot = self.web3.eth.get_storage_at("0x3100000000000000000000000000000000000002", 0)
        self.assertEqual(slot, HexBytes('0x0000000000000000000000deaddeaddeaddeaddeaddeaddeaddeaddeaddead01'))

    def test_get_code(self):
        code = self.web3.eth.get_code('0x3200000000000000000000000000000000000001')
        self.assertEqual(code, HexBytes('0x608060405234801561001057600080fd5b50600436106100a95760003560e01c806357e871e71161007157806357e871e71461014c57806361b207e214610155578063a91d8b3d14610182578063d269a03e146101a2578063d761753e146101b5578063ee82ac5e146101e857600080fd5b80630466efc4146100ae5780630e27bc11146100e15780631f578333146100f657806334cdf78d146101095780634ffd344a14610129575b600080fd5b6100ce6100bc366004610598565b60009081526002602052604090205490565b6040519081526020015b60405180910390f35b6100f46100ef3660046105b1565b610208565b005b6100f4610104366004610598565b610330565b6100ce610117366004610598565b60016020526000908152604090205481565b61013c61013736600461061c565b6103de565b60405190151581526020016100d8565b6100ce60005481565b6100ce610163366004610598565b6000908152600160209081526040808320548352600290915290205490565b6100ce610190366004610598565b60026020526000908152604090205481565b61013c6101b036600461061c565b610404565b6101d073deaddeaddeaddeaddeaddeaddeaddeaddeaddead81565b6040516001600160a01b0390911681526020016100d8565b6100ce6101f6366004610598565b60009081526001602052604090205490565b3373deaddeaddeaddeaddeaddeaddeaddeaddeaddead146102705760405162461bcd60e51b815260206004820152601f60248201527f63616c6c6572206973206e6f74207468652073797374656d2063616c6c65720060448201526064015b60405180910390fd5b60008054908190036102b65760405162461bcd60e51b815260206004820152600f60248201526e139bdd081a5b9a5d1a585b1a5e9959608a1b6044820152606401610267565b60008181526001602081905260409091208490556102d5908290610677565b60009081558381526002602090815260409182902084905581518381529081018590529081018390527f32eff959e2e8d1609edc4b39ccf75900aa6c1da5719f8432752963fdf008234f9060600160405180910390a1505050565b3373deaddeaddeaddeaddeaddeaddeaddeaddeaddead146103935760405162461bcd60e51b815260206004820152601f60248201527f63616c6c6572206973206e6f74207468652073797374656d2063616c6c6572006044820152606401610267565b600054156103d95760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610267565b600055565b6000858152600160205260408120546103fa908686868661040f565b9695505050505050565b60006103fa86868686865b6000858152600260209081526040808320548151601f870184900484028101840190925285825291610462918891849190899089908190840183828082843760009201919091525089925061046d915050565b979650505050505050565b6000838514801561047c575081155b801561048757508251155b15610494575060016104a3565b6104a0858486856104ab565b90505b949350505050565b6000602084516104bb9190610698565b156104c8575060006104a3565b83516000036104d9575060006104a3565b818560005b8651811015610548576104f2600284610698565b6001036105165761050f6105098883016020015190565b83610555565b915061052f565b61052c826105278984016020015190565b610555565b91505b60019290921c91610541602082610677565b90506104de565b5090931495945050505050565b6000610561838361056a565b90505b92915050565b60008260005281602052602060006040600060025afa50602060006020600060025afa505060005192915050565b6000602082840312156105aa57600080fd5b5035919050565b600080604083850312156105c457600080fd5b50508035926020909101359150565b60008083601f8401126105e557600080fd5b50813567ffffffffffffffff8111156105fd57600080fd5b60208301915083602082850101111561061557600080fd5b9250929050565b60008060008060006080868803121561063457600080fd5b8535945060208601359350604086013567ffffffffffffffff81111561065957600080fd5b610665888289016105d3565b96999598509660600135949350505050565b8082018082111561056457634e487b7160e01b600052601160045260246000fd5b6000826106b557634e487b7160e01b600052601260045260246000fd5b50069056'))

    def test_get_block(self):
        block = self.web3.eth.get_block('latest')
        self.assertEqual(block['number'], self.web3.eth.get_block_number())

        first_tx_receipt = self.web3.eth.get_transaction_receipt(self.first_tx_hash)
        block = self.web3.eth.get_block(first_tx_receipt['blockNumber'])
        self.assertEqual(block['hash'], first_tx_receipt['blockHash'])

        block = self.web3.eth.get_block(first_tx_receipt['blockHash'])
        self.assertEqual(block['hash'], first_tx_receipt['blockHash'])

    def test_get_transaction_count_block(self):
        # Test it with block number
        tx_count = self.web3.eth.get_block_transaction_count(1)
        self.assertEqual(tx_count, 3)
        block = self.web3.eth.get_block(1)
        # Test it with hash
        tx_count = self.web3.eth.get_block_transaction_count(block.hash)
        self.assertEqual(tx_count, 3)

    def test_get_transaction(self):
        tx = self.web3.eth.get_transaction(self.first_tx_hash)
        self.assertEqual(tx['hash'], self.first_tx_hash)
        self.assertEqual(tx['from'], "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        self.assertEqual(tx['to'], "0x0000000000000000000000000000000000000000")
    
    def test_get_transaction_by_block(self):
        tx = self.web3.eth.get_transaction(self.first_tx_hash)
        block = self.web3.eth.get_block(tx['blockHash'])
        index = tx['transactionIndex']
        block_number = block['number']
        tx = self.web3.eth.get_transaction_by_block(block_number, index)
        self.assertEqual(tx['hash'], self.first_tx_hash)

    def test_get_transaction_receipt(self):
        receipt = self.web3.eth.get_transaction_receipt(self.first_tx_hash)
        self.assertEqual(receipt['transactionHash'], self.first_tx_hash)
        self.assertEqual(receipt['from'], "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        self.assertEqual(receipt['to'], "0x0000000000000000000000000000000000000000")
        self.assertGreater(int(receipt['l1DiffSize'], 16), 0)
        self.assertGreater(int(receipt['l1FeeRate'], 16), 0)

    def test_get_transaction_count(self):
        tx_count = self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        self.assertGreater(tx_count, 0)

    def test_call(self):
        selector = self.web3.keccak(text='SYSTEM_CALLER()')[:4]
        return_val = self.web3.eth.call({'value': 0, 'to': '0x3100000000000000000000000000000000000001', 'data': selector})
        self.assertEqual(return_val, HexBytes('0x000000000000000000000000deaddeaddeaddeaddeaddeaddeaddeaddeaddead'))

    def test_create_access_list(self):        
        tx = {
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'to': "0x3100000000000000000000000000000000000002",
            'value': self.web3.to_wei(10, 'ether'),  
            'gas': 200000,
            'gasPrice': self.web3.to_wei(1, 'gwei'),
            'data': "0x8786dba712340000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000", # withdraw(bytes32, bytes4), param is 0x1234, 0x01
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'chainId': 5655
        }
        access_list = self.web3.eth.create_access_list(tx)
        # Assert existence of the access list
        self.assertGreater(len(access_list), 1)

    def test_fee_history(self):
        fee_history = self.web3.eth.fee_history(1, 1)
        self.assertGreater(fee_history['gasUsedRatio'][0], 0)

    def test_estimate_gas(self):
        estimate = self.web3.eth.estimate_gas({
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'to': "0x0000000000000000000000000000000000000000",
            'value': self.web3.to_wei(1, 'ether'),
        })
        self.assertGreaterEqual(estimate, 21000) # Extra gas comes from L1 fee addition on top of 21000, 21000 is the default Ether transfer gas amount

    def test_get_logs(self):
        # OperatorUpdated event
        logs = self.web3.eth.get_logs({'fromBlock': 1, 'toBlock': 1, 'topics': ["0xfbe5b6cbafb274f445d7fed869dc77a838d8243a22c460de156560e8857cad03"]})
        self.assertEqual(logs[0].address, "0x3100000000000000000000000000000000000002")
        # Operator is updated from 0x0 to deaddeaddeaddeaddeaddeaddeaddeaddeaddead address which is the system caller
        self.assertEqual(logs[0].data, HexBytes("0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000deaddeaddeaddeaddeaddeaddeaddeaddeaddead"))

    def test_contract_deploy(self):
        abi = [{"constant":True,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"guy","type":"address"},{"name":"wad","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":True,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"src","type":"address"},{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":False,"inputs":[{"name":"wad","type":"uint256"}],"name":"withdraw","outputs":[],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":True,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":True,"inputs":[{"name":"","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":True,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":False,"inputs":[],"name":"deposit","outputs":[],"payable":True,"stateMutability":"payable","type":"function"},{"constant":True,"inputs":[{"name":"","type":"address"},{"name":"","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"payable":True,"stateMutability":"payable","type":"fallback"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":True,"name":"guy","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":True,"name":"dst","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"dst","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Deposit","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Withdrawal","type":"event"}]
        initcode = "60606040526040805190810160405280600d81526020017f57726170706564204574686572000000000000000000000000000000000000008152506000908051906020019061004f9291906100c8565b506040805190810160405280600481526020017f57455448000000000000000000000000000000000000000000000000000000008152506001908051906020019061009b9291906100c8565b506012600260006101000a81548160ff021916908360ff16021790555034156100c357600080fd5b61016d565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1061010957805160ff1916838001178555610137565b82800160010185558215610137579182015b8281111561013657825182559160200191906001019061011b565b5b5090506101449190610148565b5090565b61016a91905b8082111561016657600081600090555060010161014e565b5090565b90565b610c348061017c6000396000f3006060604052600436106100af576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806306fdde03146100b9578063095ea7b31461014757806318160ddd146101a157806323b872dd146101ca5780632e1a7d4d14610243578063313ce5671461026657806370a082311461029557806395d89b41146102e2578063a9059cbb14610370578063d0e30db0146103ca578063dd62ed3e146103d4575b6100b7610440565b005b34156100c457600080fd5b6100cc6104dd565b6040518080602001828103825283818151815260200191508051906020019080838360005b8381101561010c5780820151818401526020810190506100f1565b50505050905090810190601f1680156101395780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b341561015257600080fd5b610187600480803573ffffffffffffffffffffffffffffffffffffffff1690602001909190803590602001909190505061057b565b604051808215151515815260200191505060405180910390f35b34156101ac57600080fd5b6101b461066d565b6040518082815260200191505060405180910390f35b34156101d557600080fd5b610229600480803573ffffffffffffffffffffffffffffffffffffffff1690602001909190803573ffffffffffffffffffffffffffffffffffffffff1690602001909190803590602001909190505061068c565b604051808215151515815260200191505060405180910390f35b341561024e57600080fd5b61026460048080359060200190919050506109d9565b005b341561027157600080fd5b610279610b05565b604051808260ff1660ff16815260200191505060405180910390f35b34156102a057600080fd5b6102cc600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610b18565b6040518082815260200191505060405180910390f35b34156102ed57600080fd5b6102f5610b30565b6040518080602001828103825283818151815260200191508051906020019080838360005b8381101561033557808201518184015260208101905061031a565b50505050905090810190601f1680156103625780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b341561037b57600080fd5b6103b0600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091908035906020019091905050610bce565b604051808215151515815260200191505060405180910390f35b6103d2610440565b005b34156103df57600080fd5b61042a600480803573ffffffffffffffffffffffffffffffffffffffff1690602001909190803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610be3565b6040518082815260200191505060405180910390f35b34600360003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600082825401925050819055503373ffffffffffffffffffffffffffffffffffffffff167fe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c346040518082815260200191505060405180910390a2565b60008054600181600116156101000203166002900480601f0160208091040260200160405190810160405280929190818152602001828054600181600116156101000203166002900480156105735780601f1061054857610100808354040283529160200191610573565b820191906000526020600020905b81548152906001019060200180831161055657829003601f168201915b505050505081565b600081600460003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040518082815260200191505060405180910390a36001905092915050565b60003073ffffffffffffffffffffffffffffffffffffffff1631905090565b600081600360008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054101515156106dc57600080fd5b3373ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff16141580156107b457507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600460008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205414155b156108cf5781600460008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020541015151561084457600080fd5b81600460008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600082825403925050819055505b81600360008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206000828254039250508190555081600360008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600082825401925050819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040518082815260200191505060405180910390a3600190509392505050565b80600360003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205410151515610a2757600080fd5b80600360003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600082825403925050819055503373ffffffffffffffffffffffffffffffffffffffff166108fc829081150290604051600060405180830381858888f193505050501515610ab457600080fd5b3373ffffffffffffffffffffffffffffffffffffffff167f7fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65826040518082815260200191505060405180910390a250565b600260009054906101000a900460ff1681565b60036020528060005260406000206000915090505481565b60018054600181600116156101000203166002900480601f016020809104026020016040519081016040528092919081815260200182805460018160011615610100020316600290048015610bc65780601f10610b9b57610100808354040283529160200191610bc6565b820191906000526020600020905b815481529060010190602001808311610ba957829003601f168201915b505050505081565b6000610bdb33848461068c565b905092915050565b60046020528160005260406000206020528060005260406000206000915091505054815600a165627a7a72305820deb4c2ccab3c2fdca32ab3f46728389c2fe2c165d5fafa07661e4e004f6c344a0029"
        contract = self.web3.eth.contract(abi=abi, bytecode=initcode)
        tx = contract.constructor().build_transaction({
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'value': 0,
            'gas': 2000000,
            'gasPrice': self.web3.to_wei(1, 'gwei'),
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'chainId': 5655
        })
        signed_tx = self.web3.eth.account.sign_transaction(tx, "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        tx_hash = self.web3.eth.send_raw_transaction(signed_tx.raw_transaction)
        receipt = self.web3.eth.wait_for_transaction_receipt(tx_hash)
        weth_address = receipt['contractAddress']

        contract = self.web3.eth.contract(address=weth_address, abi=abi)
        self.assertEqual(contract.functions.name().call(), "Wrapped Ether")
        self.assertEqual(contract.functions.balanceOf("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").call(), 0)

        # Deposit 1 Ether
        tx = contract.functions.deposit().build_transaction({
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'value': self.web3.to_wei(1, 'ether'),
            'gas': 2000000,
            'gasPrice': self.web3.to_wei(1, 'gwei'),
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'chainId': 5655
        })
        signed_tx = self.web3.eth.account.sign_transaction(tx, "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        tx_hash = self.web3.eth.send_raw_transaction(signed_tx.raw_transaction)
        receipt = self.web3.eth.wait_for_transaction_receipt(tx_hash)

        # Check if 1 WETH is minted
        self.assertEqual(contract.functions.balanceOf("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").call(), self.web3.to_wei(1, 'ether'))

    def test_call_errors_correctly(self):
        try:
            selector = self.web3.keccak(text='ERRONEUS_FUNC()')[:4]
            self.web3.eth.call({'value': 0, 'to': '0x3100000000000000000000000000000000000001', 'data': selector})
            self.fail("Expected call to fail, but it succeeded.")
        except Exception as e:
            self.assertEqual(str(e), "('execution reverted', 'no data')")

    def test_send_raw_transaction_reverts_correctly(self):
        tx = {
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'to': "0x3100000000000000000000000000000000000002",
            'value': self.web3.to_wei(0.9, 'ether'),  
            'gas': 200000,
            'gasPrice': self.web3.to_wei(1, 'gwei'),
            'data': "0x8786dba712340000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000", # withdraw(bytes32, bytes4), param is 0x1234, 0x01
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'chainId': 5655
        }
        signed_tx = self.web3.eth.account.sign_transaction(tx, "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        tx_hash = self.web3.eth.send_raw_transaction(signed_tx.raw_transaction)
        receipt = self.web3.eth.wait_for_transaction_receipt(tx_hash)
        self.assertEqual(receipt['status'], 0)

    def test_call_errors_correctly_on_withdraw(self):
        tx = {
            'from': "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            'to': "0x3100000000000000000000000000000000000002",
            'value': self.web3.to_wei(0.9, 'ether'),  
            'gas': 200000,
            'gasPrice': self.web3.to_wei(1, 'gwei'),
            'data': "0x8786dba712340000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000", # withdraw(bytes32, bytes4), param is 0x1234, 0x01
            'nonce': self.web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            'chainId': 5655
        }
        try:
            self.web3.eth.call(tx)
            self.fail("Expected call to fail, but it succeeded.")
        except Exception as e:
            self.assertEqual(str(e), "('execution reverted: revert: Invalid withdraw amount', '0x08c379a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000017496e76616c696420776974686472617720616d6f756e74000000000000000000')")


    def test_get_transaction_false_hash(self):
        try:
            self.web3.eth.get_transaction("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            self.fail("Expected get_transaction to fail, but it succeeded.")
        except Exception as e:
            self.assertEqual(str(e), "Transaction with hash: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef' not found.")

    def test_get_block_false_hash(self):
        try:
            self.web3.eth.get_block("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            self.fail("Expected get_block to fail, but it succeeded.")
        except Exception as e:
            self.assertEqual(str(e), "Block with id: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef' not found.")

    def test_get_transaction_receipt_false_hash(self):
        try:
            self.web3.eth.get_transaction_receipt("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            self.fail("Expected get_transaction_receipt to fail, but it succeeded.")
        except Exception as e:
            self.assertEqual(str(e), "Transaction with hash: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef' not found.")

    def test_get_logs_false_hash(self):
        logs = self.web3.eth.get_logs({'fromBlock': 1, 'toBlock': 1, 'topics': ["0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"]})
        self.assertEqual(logs, [])

if __name__ == '__main__':
    unittest.main()