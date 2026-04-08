class Solution:
    # set gebruikt achterliggend hashtable bijna altijd O(1), buiten wanneer er collisions zijn. Geen idee hoe ze collisions oplossen.
    # Ik denk dat ze achterliggen arr gebruiken en met hash direct naar de juiste index [bucket springen] om de daar gestoorde value op te halen. Dus op baisi van de berekend hash storen ze de value op index [hashResult] in de arr.
    def containsDuplicate(self, nums: list[int]) -> bool:
        setje = set()
        for num in nums:
            if num in setje:
                return True
            else:
                setje.add(num)
        return False
