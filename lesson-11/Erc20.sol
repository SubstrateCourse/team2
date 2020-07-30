pragma solidity ^0.6.0;
import "./SafeMath.sol";

contract Erc20 {
    using SafeMath for uint256;

    //total Supply
    uint256 private _totalSupply;
    //balance of user
    mapping(address => uint256) _balances;
    //allowance
    mapping(address => mapping(address => uint256)) private _allawance;
    string private _name;
    string private _sysmbol;
    uint8 private _decimals;

    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(string memory _myName, string memory _mysymbol, uint8 _mydecimals, uint256 _myTotalSupply) public {
        _name = _myName;
        _sysmbol = _mysymbol;
        _decimals = _mydecimals;
        _totalSupply = _myTotalSupply;
    }

    function name() public view returns (string memory) {
        return _name;
    }

    function  symbol() public view returns (string memory) {
        return _sysmbol;
    }

    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function decimals() public view returns (uint8) {
        return _decimals;
    }

    function balanceOf(address account) public view returns (uint256) {
        return _balances[account];
    }

    function allowance(address owner, address spender) public view returns (uint256) {
        return _allawance[owner][spender];
    }


    function  transfer(address to, uint256 value) public returns (bool) {
        return _transfer(msg.sender, to, value);
    }


    function  _transfer(address from, address to, uint256 value) public returns (bool) {
        require(from != address(0), "Erc20: transfer from zero address");
        require(to != address(0), "Erc20: transfer to zero address");

        _balances[from] = _balances[from].sub(value);
        _balances[to] = _balances[to].add(value);

        emit Transfer(from, to, value);
        return true;
    }

    function  transferFrom(address owner, address to, uint256 value) public returns (bool) {
        require(_allawance[owner][msg.sender] >= value, "Erc20: not allowed to transferFrom");
        return _transfer(owner, to, value);
    }

}