# Password Generator

A simple API for generation passwords.
You can choose:
1. The `length` of the password
2. If you want `numbers` or not
3. If you want `symbold` or not

*Lower and Upper caseses can't be toggled off.*
***

## Frontend
...

***

## Backend
Calling the base url processes the request as such:
```json
{
    length: 30,
    put_numbers: true,
    put_symbols: true,
}
```
You can put your unique request thorough: `/config?length=<int>&put_numbers=<bool>&put_symbols=<bool>`
***

RenaMice.